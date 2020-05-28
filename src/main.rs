// mod ownership;
// mod references;
// mod slice;
// mod structs;
// mod enums;
// mod packages_crates;
// mod collections;

mod generics;

fn main() {
    // ownership::string_example();
    // ownership::move_string();
    // ownership::clone_string();
    // ownership::function_example();

    // references::basic_reference();
    // references::basic_borrow();
    // references::mutable_wrong();
    // references::mutable_scope();

    // slice::first_word_independent(&s);
    // slice::first_word_example();
    // slice::slice_parameters();
    // slice::other_slices();
    // structs::basic_struct();
    // structs::rectangles();
    // structs::good_struct();
    // structs::method_struct();
    // structs::more_params();
    // structs::associated_functions();
    // enums::pattern_bind();

    // collections::vector::vector_example();
    // collections::vector::vector_read();
    // collections::vector::enforces_ownership();
    // collections::vector::iter_values();
    // collections::strings::create_string();
    // collections::strings::update_string();
    // collections::strings::concat_string();
    // collections::strings::concat_multiple();
    // collections::strings::slice_example();
    // collections::strings::best_slice();
    // collections::strings::slice_bytes();
    // collections::hash_maps::itr_collect();
    // collections::hash_maps::ownership();
    // collections::hash_maps::accessing_values();
    // collections::hash_maps::insert_on_no_value();
    // collections::hash_maps::update_on_old_value();

    // generics::remove_duplication::extract_function_example();
    // generics::remove_duplication::generic_data_types();
    // generics::struct_def::example();
    // generics::method_def::example();
    // generics::method_def::mixup();
    // generics::traits::tweet_example();
    // generics::traits::article_default();
    // generics::traits::default::example();
    generics::lifetimes::prevent_dangling_ptr();
}
