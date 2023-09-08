use md2::Md2;
use md4::Md4;
use sha1::Sha1;
use sha2::{Sha256, Digest, Sha512};

use crate::simple_form::SimpleForm;

type Process = &'static (dyn (Fn(&str) -> String) + Sync + 'static);

type SimpleFormPage = SimpleForm<Process>;

pub static MD5: SimpleFormPage = crate::simple_form::SimpleForm {
    title: "MD5 Encode",
    heading: "Hash a string using MD5",
    root_route: "md5",
    action_route: "encode",
    input_placholder: "Your string to hash...",
    action_button_text: "Compute",
    result_table_header: "MD5 Hash",
    process: &|x| format!("{:x}", md5::compute(x)),
};

pub static MD2: SimpleFormPage = crate::simple_form::SimpleForm {
    title: "MD2 Encode",
    heading: "Hash a string using MD2",
    root_route: "md2",
    action_route: "encode",
    input_placholder: "Your string to hash...",
    action_button_text: "Compute",
    result_table_header: "MD2 Hash",
    process: &|x| {
        let mut hasher = Md2::new();
        hasher.update(x);
        let result = hasher.finalize();
        format!("{:x}", result)
    },
};

pub static MD4: SimpleFormPage = crate::simple_form::SimpleForm {
    title: "MD4 Encode",
    heading: "Hash a string using MD4",
    root_route: "md4",
    action_route: "encode",
    input_placholder: "Your string to hash...",
    action_button_text: "Compute",
    result_table_header: "MD4 Hash",
    process: &|x| {
        let mut hasher = Md4::new();
        hasher.update(x);
        let result = hasher.finalize();
        format!("{:x}", result)
    },
};

pub static SHA512: SimpleFormPage = crate::simple_form::SimpleForm {
    title: "SHA-512 Encode",
    heading: "Hash a string using SHA-512",
    root_route: "sha512",
    action_route: "encode",
    input_placholder: "Your string to hash...",
    action_button_text: "Compute",
    result_table_header: "SHA-512 Hash",
    process: &|x| {
        let mut hasher = Sha512::new();
        hasher.update(x);
        let result = hasher.finalize();
        format!("{:x}", result)
    },
};

pub static SHA256: SimpleFormPage = crate::simple_form::SimpleForm {
    title: "SHA-256 Encode",
    heading: "Hash a string using SHA-256",
    root_route: "sha256",
    action_route: "encode",
    input_placholder: "Your string to hash...",
    action_button_text: "Compute",
    result_table_header: "SHA-256 Hash",
    process: &|x| {
        let mut hasher = Sha256::new();
        hasher.update(x);
        let result = hasher.finalize();
        format!("{:x}", result)
    },
};

pub static SHA1: SimpleFormPage = crate::simple_form::SimpleForm {
    title: "SHA-1 Encode",
    heading: "Hash a string using SHA-1",
    root_route: "sha1",
    action_route: "encode",
    input_placholder: "Your string to hash...",
    action_button_text: "Compute",
    result_table_header: "SHA-1 Hash",
    process: &|x| {
        let mut hasher = Sha1::new();
        hasher.update(x);
        let result = hasher.finalize();
        format!("{:x}", result)
    },
};
