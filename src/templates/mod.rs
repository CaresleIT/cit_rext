pub fn route_template(value: &String) -> String {
    let data = format!(r#"
export default function {}() {{
    return <></>
}}
    "#, value);

    data
}