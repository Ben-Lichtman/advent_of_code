pub struct Min;
pub struct Max;

pub struct SortedArr<T, const N: usize>([T; N]);

impl<T, const N: usize> SortedArr<T, N>
where
	T: Ord,
{
	pub fn from<F, O>(mut arr: [T; N], f: F) -> Self
	where
		F: FnMut(&T) -> O,
		O: Ord,
	{
		arr.sort_by_key(f);
		Self(arr)
	}

	pub fn insert<F, B>(&mut self, item: T, mut f: F)
	where
		F: FnMut(&T) -> B,
		B: Ord,
	{
		match self.0.binary_search_by_key(&f(&item), f) {
			Ok(index) | Err(index) => match self.0.get_mut(index + 1..) {
				None => (),
				Some(arr) => {
					arr.rotate_right(1);
					match arr {
						[] => (),
						[head, ..] => *head = item,
					}
				}
			},
		}
	}

	pub fn into_inner(self) -> [T; N] { self.0 }
}
