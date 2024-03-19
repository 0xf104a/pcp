use crate::progress::ProgressDisplay;

pub struct DummyProgress{}

impl ProgressDisplay for DummyProgress{
    fn new() -> Self where Self: Sized {
        DummyProgress{}
    }

    #[inline(always)]
    fn set_progress(&mut self, _status: &str, _bytes_out: usize) {
        /*stub*/
    }

    #[inline(always)]
    fn update_status(&mut self, _new_status: &str) {
        /*stub*/
    }

    #[inline(always)]
    fn add_bytes_written(&mut self, _bytes_written: usize) {
        /*stub*/
    }

    #[inline(always)]
    fn set_size(&mut self, _bytes_total: usize) {
        /*stub*/
    }

    #[inline(always)]
    fn flush(&self) {
        /*stub*/
    }
}