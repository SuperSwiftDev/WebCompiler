
// // ————————————————————————————————————————————————————————————————————————————
// // BASICS
// // ————————————————————————————————————————————————————————————————————————————

// use std::str::FromStr;

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum Role {
//     System,
//     Assistant,
//     User,
// }

// impl Role {
//     pub fn is_system(&self) -> bool {
//         match self {
//             Self::System => true,
//             _ => false,
//         }
//     }
//     pub fn is_assistant(&self) -> bool {
//         match self {
//             Self::Assistant => true,
//             _ => false,
//         }
//     }
//     pub fn is_user(&self) -> bool {
//         match self {
//             Self::User => true,
//             _ => false,
//         }
//     }
// }

// impl FromStr for Role {
//     type Err = ParseRoleError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let normalized = s.trim().to_ascii_lowercase();
//         match normalized.as_str() {
//             "system" => Ok(Role::System),
//             "assistant" => Ok(Role::Assistant),
//             "user" => Ok(Role::User),
//             _ => Err(ParseRoleError { given: s.to_string() })
//         }
//     }
// }

// #[derive(Debug, Clone)]
// pub struct ParseRoleError {
//     pub given: String,
// }

// impl std::fmt::Display for ParseRoleError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "unrecognized role {:?}", self.given)
//     }
// }

// impl std::error::Error for ParseRoleError {}

// impl crate::xm
// impl DslFormatError for ParseRoleError {
//     fn singleton(&self) -> DslFormatErrorList { DslFormatErrorList::new(Rc::new(self.clone())) }
// }

