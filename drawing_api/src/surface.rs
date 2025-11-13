pub trait Surface {
    type DisplayList;

    fn draw(&self, display_list: &Self::DisplayList) -> Result<(), &'static str>;
}
