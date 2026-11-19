#![forbid(unsafe_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#[allow(unused_imports)]
use binder::binder_impl::BorrowedParcel;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct r#RFNodeStatus {
    pub r#status: i32,
    pub r#value: i32,
}

impl Default for r#RFNodeStatus {
    fn default() -> Self {
        Self {
            r#status: Default::default(),
            r#value: Default::default(),
        }
    }
}

impl binder::Parcelable for r#RFNodeStatus {
    fn write_to_parcel(
        &self,
        parcel: &mut BorrowedParcel<'_>,
    ) -> std::result::Result<(), binder::StatusCode> {
        parcel.sized_write(|subparcel| {
            subparcel.write(&self.r#status)?;
            subparcel.write(&self.r#value)?;
            Ok(())
        })
    }

    fn read_from_parcel(
        &mut self,
        parcel: &BorrowedParcel<'_>,
    ) -> std::result::Result<(), binder::StatusCode> {
        parcel.sized_read(|subparcel| {
            if subparcel.has_more_data() {
                self.r#status = subparcel.read()?;
            }
            if subparcel.has_more_data() {
                self.r#value = subparcel.read()?;
            }
            Ok(())
        })
    }
}

binder::impl_serialize_for_parcelable!(r#RFNodeStatus);
binder::impl_deserialize_for_parcelable!(r#RFNodeStatus);
impl binder::binder_impl::ParcelableMetadata for r#RFNodeStatus {
    fn get_descriptor() -> &'static str {
        "vendor.oplus.hardware.engineer.RFNodeStatus"
    }
    fn get_stability(&self) -> binder::binder_impl::Stability {
        binder::binder_impl::Stability::Vintf
    }
}
