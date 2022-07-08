use super::*;

pub mod v2 {
    use super::*;

    pub fn migrate<T: Config>() -> Weight {

        StorageVersion::<T>::put(Releases::V2_0_0);
        Something2::<T>::translate::<u32, _>(|p| Some(InputS { data: p.unwrap_or(0)}));
        T::DbWeight::get().reads_writes(1, 2)
    }
}