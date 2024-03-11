use optimizoor_ast::{
    ContractPart, ElementaryTypeName, Program, SourceUnit, StructDefinition, TypeName,
    TypeNameNode, VariableDeclaration,
};
use regex::Regex;
use std::str::FromStr;

/// Struct Packing Optimisation

/// Optimizes the packing of all structs in the program.
pub fn optimize_structs(program: &mut Program<'_>) {
    for source_unit in program.body().iter() {
        match source_unit.value {
            SourceUnit::ContractDefinition(contract_def) => {
                // get the struct definitions
                let mut struct_defs = contract_def // contract_def is a ContractDefinition<'_>
                    .body
                    .iter()
                    .filter_map(|part| match part.value {
                        ContractPart::StructDefinition(struct_def) => {
                            let mut struct_def = struct_def;
                            optimize_struct_definition(&mut struct_def);
                            Some(struct_def)
                        }
                        _ => None,
                    });
            }
            SourceUnit::PragmaDirective(_) => {}
            SourceUnit::ImportDirective(_) => {}
        }
    }
}

/// function to optimize a single struct
fn optimize_struct_definition(struct_def: &mut StructDefinition) {
    let struct_body = struct_def.body;
    // map them into field type
    let mut fields: Vec<Field> = struct_body
        .iter()
        .map(|field| {
            let data_type = to_data_type(&field.value.type_name.value);
            let size = data_type.size();
            Field {
                name: field.value.id.value.to_string(),
                ty: data_type,
                size: size,
            }
        })
        .collect();

    // Sort fields in decreasing order of size
    fields.sort_by(|a, b| b.size.cmp(&a.size));
}

/// Performs a bin-packing algorithm to pack fields into storage slots.
/// Given a list of fields sorted by size and existing storage slots, it tries to fit the fields
/// into the slots in a way that minimizes the number of slots used. If a field cannot fit into
/// any existing slot, a new slot is created. The function is recursive and considers all possible
/// packing combinations to find the optimal result.
fn bin_packing(fields: Vec<Field>, existing_slots: Vec<StorageSlot>) -> Vec<StorageSlot> {
    if fields.is_empty() {
        return existing_slots;
    }

    let current_item = &fields[0];
    let mut packing_options: Vec<Vec<StorageSlot>> = Vec::new();

    for (i, slot) in existing_slots.iter().enumerate() {
        if slot.offset + current_item.size > 32 {
            continue;
        }

        let mut slots_copy = existing_slots.clone();
        slots_copy[i].fields.push(current_item.clone());
        slots_copy[i].offset += current_item.size;

        packing_options.push(bin_packing(fields[1..].to_vec(), slots_copy));
    }

    packing_options.push(bin_packing(fields[1..].to_vec(), {
        let mut new_slot = existing_slots.clone();
        new_slot.push(StorageSlot {
            fields: vec![current_item.clone()],
            offset: current_item.size,
        });
        new_slot
    }));

    find_optimal_packing(packing_options)
}

/// Finds the most optimal packing arrangement from a set of packing options.
fn find_optimal_packing(options: Vec<Vec<StorageSlot>>) -> Vec<StorageSlot> {
    if options.is_empty() {
        return vec![];
    }

    let mut best_slots = &options[0];
    let mut least_amount_of_slots = best_slots.len();
    let mut most_filled_storage_slots = count_filled_slots(best_slots);

    for option in &options {
        let filled_slots = count_filled_slots(option);
        if option.len() < least_amount_of_slots
            || (option.len() == least_amount_of_slots && filled_slots > most_filled_storage_slots)
        {
            best_slots = option;
            least_amount_of_slots = option.len();
            most_filled_storage_slots = filled_slots;
        }
    }

    best_slots.clone()
}

/// Counts the number of fully filled storage slots in a given set of slots.
fn count_filled_slots(slots: &[StorageSlot]) -> usize {
    slots.iter().filter(|slot| slot.offset == 32).count()
}

/// Extracts and transforms fields from a struct definition into a vector of `Field` objects.
fn get_fields_from_definition(struct_def: &StructDefinition) -> Vec<Field> {
    struct_def
        .body
        .iter()
        .map(|field| {
            let data_type = to_data_type(&field.value.type_name.value);
            Field {
                name: field.value.id.value.to_string(),
                ty: data_type.clone(),
                size: data_type.size(),
            }
        })
        .collect()
}

/// Applies the optimized packing order of fields back to the struct definition.
fn apply_packing_to_definition(struct_def: &mut StructDefinition, packed_slots: Vec<StorageSlot>) {
    // Flatten the fields from the storage slots while maintaining the new order
    let packed_fields: Vec<Field> = packed_slots
        .into_iter()
        .flat_map(|slot| slot.fields)
        .collect();

    // How do i convert back with new updated location? the id would shift as well
}

// Converts from AST type to field type for sorting. Only for struct packing now
fn to_data_type(ty: &TypeName) -> DataType {
    match ty {
        TypeName::ElementaryTypeName(name) => match name {
            ElementaryTypeName::Address => DataType::Address,
            ElementaryTypeName::Bool => DataType::Bool,
            ElementaryTypeName::String => DataType::String,
            ElementaryTypeName::Bytes => DataType::Bytes(None),
            ElementaryTypeName::Int(size) => DataType::Int(Some(*size)),
            ElementaryTypeName::Uint(size) => DataType::Uint(Some(*size)),
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
        // TypeName::UserDefinedTypeName(name) => DataType::String,
        // TypeName::Mapping(_) => DataType::String, // Unimplemented
        // TypeName::ArrayTypeName => DataType::String, // Unimplemented
        // TypeName::FunctionTypeName => DataType::String, // Unimplemented
    }
}

// Converts from Data type to AST type after sorting.
fn to_type_name(ty: &DataType) -> TypeName {
    match ty {
        DataType::Address => TypeName::ElementaryTypeName(ElementaryTypeName::Address),
        DataType::Bool => TypeName::ElementaryTypeName(ElementaryTypeName::Bool),
        DataType::String => TypeName::ElementaryTypeName(ElementaryTypeName::String),
        DataType::Bytes(None) => TypeName::ElementaryTypeName(ElementaryTypeName::Bytes),
        DataType::Int(Some(size)) => TypeName::ElementaryTypeName(ElementaryTypeName::Int(*size)),
        DataType::Uint(Some(size)) => TypeName::ElementaryTypeName(ElementaryTypeName::Uint(*size)),
        _ => unimplemented!(),
    }
}

/// Data types for struct fields
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum DataType {
    String,
    Bytes(Option<u8>), // None for "bytes", Some(size) for "bytes<size>"
    Bool,
    Int(Option<u8>),  // None for "int", Some(size) for "int<size>"
    Uint(Option<u8>), // None for "uint", Some(size) for "uint<size>"
    Address,
}

impl FromStr for DataType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let int_regex = Regex::new(r"u?int(\d+)").unwrap();
        let bytes_regex = Regex::new(r"bytes(\d+)").unwrap();

        match s {
            "string" => Ok(DataType::String),
            "bytes" => Ok(DataType::Bytes(None)),
            "bool" => Ok(DataType::Bool),
            "int" => Ok(DataType::Int(None)),
            "uint" => Ok(DataType::Uint(None)),
            "address" => Ok(DataType::Address),
            _ if s.ends_with("[]") => Ok(DataType::String), // Assuming the type is dynamic array
            _ if int_regex.is_match(s) => {
                let captures = int_regex.captures(s).unwrap();
                let size = captures.get(1).unwrap().as_str().parse::<u8>().unwrap();
                if s.starts_with("uint") {
                    Ok(DataType::Uint(Some(size)))
                } else {
                    Ok(DataType::Int(Some(size)))
                }
            }
            _ if bytes_regex.is_match(s) => {
                let captures = bytes_regex.captures(s).unwrap();
                let size = captures.get(1).unwrap().as_str().parse::<u8>().unwrap();
                Ok(DataType::Bytes(Some(size)))
            }
            _ => Err(()), // Unknown type
        }
    }
}

impl DataType {
    fn size(&self) -> u8 {
        match self {
            DataType::String | DataType::Bytes(None) => 32,
            DataType::Bool => 1,
            DataType::Int(None) | DataType::Uint(None) | DataType::Address => 32,
            DataType::Int(Some(size)) | DataType::Uint(Some(size)) => size / 8,
            DataType::Bytes(Some(size)) => *size,
        }
    }

    fn clone(&self) -> DataType {
        match self {
            DataType::String => DataType::String,
            DataType::Bytes(size) => DataType::Bytes(*size),
            DataType::Bool => DataType::Bool,
            DataType::Int(size) => DataType::Int(*size),
            DataType::Uint(size) => DataType::Uint(*size),
            DataType::Address => DataType::Address,
        }
    }
}

fn main() {
    let data_type = "uint256".parse::<DataType>().expect("Invalid data type");
    println!("The size of {:?} is {} bytes.", data_type, data_type.size());
}

/// Field is a struct that represents a field in a struct
#[derive(Clone)]
struct Field {
    name: String,
    ty: DataType,
    size: u8, // Size of the field in bytes
}

/// StorageSlot is a struct that represents a storage slot in a struct.
/// The max size of a storage slot is 32 bytes
#[derive(Clone)]
struct StorageSlot {
    fields: Vec<Field>,
    offset: u8, // Offset of the storage slot in the struct i.e How many bytes before 32 bytes slot hits.
}
