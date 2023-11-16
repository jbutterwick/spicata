use leptos::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    view! {

        <form method="post" enctype="multipart/form-data">
          <div>
            <label for="transaction-data">Upload transaction data:</label>
            <input
              type="file"
              id="transaction-data"
              name="transaction-data"
              accept=".csv"
            />
          </div>
          <div class="preview">
            <p>No files currently selected for upload</p>
          </div>
          <div>
            <button>Submit</button>
          </div>
        </form>
    }
}

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}