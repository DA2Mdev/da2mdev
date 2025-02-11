use yew::{function_component, html, Html};

#[function_component]
pub fn HomeStandard() -> Html {
    html! {
      <div class="w-screen h-screen bg-zinc-100 dark:bg-zinc-900 grid items-center">
        <div class="mx-25">
          <h1 class="text-6xl flex flex-row items-center my-5">
            {"David Maestre"}
            <div class="flex flex-row gap-x-2 bg-zinc-900 dark:bg-zinc-100 ml-5 p-1 rounded-full">
              <a
                target="_blank"
                href="http://github.com/DA2Mdev"
                class="flex items-center gap-x-2 bg-zinc-100 dark:bg-zinc-900 rounded-full px-5 py-3 text-4xl"
               >
                <img class="h-6 w-6" src="img/github-mark-white.svg" alt="github_icon" />
                {"DA2Mdev"}
              </a>
              <a
                target="_blank"
                href="http://www.linkedin.com/in/david-maestre-734839210"
                class="flex items-center gap-x-2 px-6 bg-zinc-100 dark:bg-zinc-900 rounded-full text-4xl py-3"
               >
                <img class="h-7 w-7" src="img/linkedin-mark-white.svg" alt="linkedin_icon" />
                {"Profile"}
              </a>
            </div>
          </h1>
          <div class="mx-5 flex flex-start">
            <span class="text-2xl font-bold italic">{"</>"}</span>
            <p class="text-lg mx-5">
              {"Hola mundo, mi nombre es David Maestre, soy un pequeño programador apasionado por la tecnologia ..."}
            </p>
          </div>
        </div>
      </div>
    }
}
