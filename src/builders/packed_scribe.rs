pub trait PackedScribe {
    fn add_int32(&mut self, value: i32) -> &mut Self;
    fn add_int64(&mut self, value: i64) -> &mut Self;
    fn add_uint32(&mut self, value: u32) -> &mut Self;
    fn add_uint64(&mut self, value: u64) -> &mut Self;
    fn add_bool(&mut self, value: bool) -> &mut Self;
    fn add_enum(&mut self, value: i32) -> &mut Self;
    fn add_sint32(&mut self, value: i32) -> &mut Self;
    fn add_sint64(&mut self, value: i64) -> &mut Self;

    fn add_fixed32(&mut self, value: u32) -> &mut Self;
    fn add_sfixed32(&mut self, value: i32) -> &mut Self;
    fn add_float(&mut self, value: f32) -> &mut Self;

    fn add_fixed64(&mut self, value: u64) -> &mut Self;
    fn add_sfixed64(&mut self, value: i64) -> &mut Self;
    fn add_double(&mut self, value: f64) -> &mut Self;
}
