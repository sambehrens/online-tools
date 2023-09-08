use crate::simple_form::SimpleForm;

pub static MD5: SimpleForm<&'static (dyn (Fn(&str) -> String) + Sync + 'static)> = crate::simple_form::SimpleForm {
        title: "MD5 Encode",
        heading: "Hash a string using MD5",
        root_route: "md5",
        action_route: "encode",
        input_placholder: "Your string to hash...",
        action_button_text: "Compute",
        result_table_header: "MD5 Hash",
        process: &|x| format!("{:x}", md5::compute(x)),
};

pub static SHA1: SimpleForm<&'static (dyn (Fn(&str) -> String) + Sync + 'static)> = crate::simple_form::SimpleForm {
        title: "SHA1 Encode",
        heading: "Hash a string using SHA1",
        root_route: "sha1",
        action_route: "encode",
        input_placholder: "Your string to hash...",
        action_button_text: "Compute",
        result_table_header: "SHA1 Hash",
        process: &|x| format!("{:x}", md5::compute(x)),
};
