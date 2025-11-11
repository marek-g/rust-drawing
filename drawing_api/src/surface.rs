pub trait Surface {
    type DisplayList;

    fn draw(&mut self, display_list: &Self::DisplayList) -> Result<(), &'static str>;
}
