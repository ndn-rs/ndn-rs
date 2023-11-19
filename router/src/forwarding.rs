use super::*;

#[derive(Debug, Default)]
pub struct ForwardingInformationBase;

impl ForwardingInformationBase {
    pub async fn lookup(&self, interest: &Interest) -> FaceRef {
        println!("Looking up forward face for {:?}", interest);
        todo!()
    }
}
