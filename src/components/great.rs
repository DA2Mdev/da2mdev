use yew::{function_component, html, Html};

#[function_component]
pub fn Great() -> Html {
    html! {
        <div>
            <h1 class="text-4xl flex flex-row items-center">
                {"David Maestre /"} 
                <a target="_blank" href="http://github.com/DA2Mdev" class="flex items-center ml-3 gap-x-2">
                    <img class="h-8 w-8" src="img/github-mark.svg" alt="github_icon" />
                    {"DA2Mdev"}
                </a>
                <a target="_blank" href="http://www.linkedin.com/in/david-maestre-734839210" class="flex items-center ml-4 gap-x-3">
                    <img class="h-8 w-8" src="img/linkedin-mark-white.svg" alt="linkedin_icon" />
                    {"Profile"}
                </a>
            </h1>
            <p>{"Hola mundo, mi nombre es David Maestre, soy un pequeño programador apasionado por la tecnologia ..."}</p>
        </div>
    }
}
