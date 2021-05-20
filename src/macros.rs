macro_rules! ciboulette_selector_utils {
	($name:ident, $selected_type:ident) => {
		impl std::ops::Deref for $name {
			type Target = crate::CibouletteSelector<$selected_type>;

			/// Get the inner type
			fn deref(&self) -> &Self::Target {
				&self.0
			}
		}

		impl $name {
			/// Create a new selector
			pub fn new(val: crate::CibouletteSelector<$selected_type>) -> Self {
				$name(val)
			}

			/// Take the selector, consuming the object
			pub fn take(self) -> crate::CibouletteSelector<$selected_type> {
				self.0
			}
		}

	};

	($name:ident, $selected_type:ident, $($bound:tt),+) => {
		impl<$($bound),+> std::ops::Deref for $name<$($bound),+> {
			type Target = crate::CibouletteSelector<$selected_type<$($bound),+>>;

			/// Get the inner type
			fn deref(&self) -> &Self::Target {
				&self.0
			}
		}

		impl<$($bound),+> $name<$($bound),+> {
			/// Create a new selector
			pub fn new(val: crate::CibouletteSelector<$selected_type<$($bound),+>>) -> Self {
				$name(val)
			}

			/// Take the selector, consuming the object
			pub fn take(self) -> crate::CibouletteSelector<$selected_type<$($bound),+>> {
				self.0
			}
		}
	};
}

// macro_rules! ciboulette_selector_deserialize {
// 	($name:ident, $selected_type:ident) => {
// 		impl std::ops::Deref for $name {
// 			type Target = crate::CibouletteSelector<$selected_type>;

// 			/// Get the inner type
// 			fn deref(&self) -> &Self::Target {
// 				&self.0
// 			}
// 		}

// 		impl $name {
// 			/// Create a new selector
// 			pub fn new(val: crate::CibouletteSelector<$selected_type>) -> Self {
// 				$name(val)
// 			}
// 		}
// 	};

// 	// ($name:ident, $selected_type:ident, $($bound:tt),+) => {
// 	// 	impl<$($bound),+> std::ops::Deref for $name<$($bound),+> {
// 	// 		type Target = crate::CibouletteSelector<$selected_type<$($bound),+>>;

// 	// 		/// Get the inner type
// 	// 		fn deref(&self) -> &Self::Target {
// 	// 			&self.0
// 	// 		}
// 	// 	}

// 	// 	impl<$($bound),+> $name<$($bound),+> {
// 	// 		/// Create a new selector
// 	// 		pub fn new(val: crate::CibouletteSelector<$selected_type<$($bound),+>>) -> Self {
// 	// 			$name(val)
// 	// 		}
// 	// 	}
// 	// };
// }
