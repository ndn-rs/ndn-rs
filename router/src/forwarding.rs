use super::*;

#[derive(Debug, Default)]
pub struct ForwardingInformationBase;

impl ForwardingInformationBase {
    pub async fn lookup(&self, interest: &Interest) -> face::FaceId {
        println!("Looking up forward face for {:?}", interest);
        todo!()
    }
}
