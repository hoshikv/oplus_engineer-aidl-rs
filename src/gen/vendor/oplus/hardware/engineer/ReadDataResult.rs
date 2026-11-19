#![forbid(unsafe_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#[allow(unused_imports)]
use binder::binder_impl::BorrowedParcel;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct r#ReadDataResult {
    pub r#result: i32,
    pub r#data: Vec<u8>,
}

impl Default for r#ReadDataResult {
    fn default() -> Self {
        Self {
            r#result: Default::default(),
            r#data: Default::default(),
        }
    }
}

impl binder::Parcelable for r#ReadDataResult {
    fn write_to_parcel(
        &self,
        parcel: &mut BorrowedParcel<'_>,
    ) -> std::result::Result<(), binder::StatusCode> {
        parcel.sized_write(|subparcel| {
            subparcel.write(&self.r#result)?;
            subparcel.write(&self.r#data)?;
            Ok(())
        })
    }

    fn read_from_parcel(
        &mut self,
        parcel: &BorrowedParcel<'_>,
    ) -> std::result::Result<(), binder::StatusCode> {
        parcel.sized_read(|subparcel| {
            if subparcel.has_more_data() {
                self.r#result = subparcel.read()?;
            }
            if subparcel.has_more_data() {
                self.r#data = subparcel.read()?;
            }
            Ok(())
        })
    }
}

binder::impl_serialize_for_parcelable!(r#ReadDataResult);
binder::impl_deserialize_for_parcelable!(r#ReadDataResult);
impl binder::binder_impl::ParcelableMetadata for r#ReadDataResult {
    fn get_descriptor() -> &'static str {
        "vendor.oplus.hardware.engineer.ReadDataResult"
    }
    fn get_stability(&self) -> binder::binder_impl::Stability {
        binder::binder_impl::Stability::Vintf
    }
}
