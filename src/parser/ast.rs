use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub struct LinkedTo {
    pub name: String,
    pub uuid: Uuid,
}

#[derive(Debug, PartialEq)]
pub enum PropValue {
    String(String),
    Integer(i64),
    Double(f64),
    Boolean(bool),
    Uuid(Uuid),
    NslocText(String, String, String),
    ObjectReference(String, String),
    LinkedToList(Vec<LinkedTo>),
    PropList(Vec<Prop>),
    Other(String),
}

#[derive(Debug, PartialEq)]
pub enum ObjectElement {
    Prop(Prop),
    CustomProp(CustomProp),
    Object(Object),
}

#[derive(Debug, PartialEq)]
pub struct Prop {
    pub key: String,
    pub value: PropValue,
}

#[derive(Debug, PartialEq)]
pub enum CustomPropValue {
    Pin(Vec<Prop>),
}

#[derive(Debug, PartialEq)]
pub struct CustomProp {
    pub domain: String,
    pub value: CustomPropValue,
}

#[derive(Debug, PartialEq)]
pub struct ObjectHeader {
    pub object_type: String,
    pub header_props: Vec<Prop>,
}

#[derive(Debug, PartialEq)]
pub struct Object {
    pub header: ObjectHeader,
    pub elements: Vec<ObjectElement>,
}

#[derive(Debug, PartialEq)]
pub struct ObjectEnd {
    pub object_type: String,
}
