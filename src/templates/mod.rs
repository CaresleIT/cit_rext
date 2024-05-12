pub fn route_template(value: &String) -> String {
    let data = format!(r#"
export default function {}() {{
    return <></>
}}
    "#, value);

    data
}

pub fn api_template() -> String {
    r#"
export default function GET(req) {}
export default function POST(req) {}
export default function PUT(req) {}
export default function DELETE(req) {}
    "#.to_string()
}

pub fn schema_template() -> String {
    r#"
import {z} from 'zod'

const SchemaCreate = z.object()

const SchemaUpdate = SchemaCreate.extends({})

const SchemaDelete = SchemaUpdate.take({})
    "#.to_string()
}

pub fn queries_template() -> String {
    r#"

const sqlServer = {
    get: {},
    post: {},
    put: {},
    remove: {},
}

const Queries = {
    sqlServer,
}

export default Queries
    "#.to_string()
}