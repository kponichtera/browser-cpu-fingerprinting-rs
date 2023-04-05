use web_sys::HtmlInputElement;
use yew::{Context, Html, html};
use yew::prelude::*;
use yew_bootstrap::component::*;

use crate::gui::app_root::AppRoot;
use crate::gui::app_root::AppRootMessage;
use crate::gui::components::*;

pub fn render_main_container(
    model_input: &str,
    input_disabled: bool,
    ctx: &Context<AppRoot>,
    button_disabled: bool,
    current_progress: f32,
    status_label: &str,
) -> Html {
    html! {
        <Container>
            {render_header()}
            {render_cpu_model_instructions(model_input.to_string(), input_disabled, ctx)}
            {render_benchmark_instructions(ctx, button_disabled)}
            {render_progress_bar(current_progress, status_label.to_string())}
        </Container>
    }
}

fn render_header() -> Html {
    html! {
        <>
            <h1 style={"text-align: center; padding-top: 3rem"}>
                { "RUST WASM CPU fingerprinting" }
            </h1>
            <p style={"margin: 2rem"}>
                { "This site will run a few JavaScript benchmarks to gather information
                about your CPU. This results of these benchmarks will then be uploaded
                to our server, where they are then stored in a database. Please click " }
                <code>{ "Continue" }</code>
                { " and follow the short instructions on the next page.
                For more information check out the FAQ at the bottom." }
            </p>
            <p style="padding-left: 2rem; padding-right: 2rem">
                { "Our benchmarks are designed for the latest versions of " }
                <strong>{ "Firefox" }</strong>
                { " and " }
                <strong>{ "Chrome" }</strong>
                { "-based browsers (e.g. Google Chrome, Chromium, Microsoft Edge etc.).\
                Most importantly Safari and iOS devices are not supported." }
            </p>
        </>
    }
}

pub fn render_cpu_model_instructions(model_input: String, input_disabled: bool, ctx: &Context<AppRoot>) -> Html {
    html! {
        <>
            <h5 style="padding-left: 2rem; padding-right: 2rem; padding-top: 3rem">
                { "Step 1 - Determining your CPU model." }
            </h5>
            <p style="padding-left: 2rem; padding-right: 2rem">
                { "Please fill out the text field below with your CPU model.
                To keep our database consistent please use the following
                method to determine your CPU model." }
            </p>
            <Accordion style="padding-left: 2rem; padding-right: 2rem" id="modelMethodsAccordion">
                <AccordionItem>
                    <AccordionHeader id="headingOne" data_bs_target="#collapseOne">
                        { "Windows" }
                    </AccordionHeader>
                    <AccordionCollapse data_bs_parent="#modelMethodsAccordion" id="collapseOne">
                        <ol>
                            <li>
                                { "Press "}
                                <kbd>{ "win" }</kbd>
                                { " + " }
                                <kbd>{ "r" }</kbd>
                                { ". This should open a small window in the lower left corner." }
                            </li>
                            <li>
                                { "Type " }
                                <code>{ "cmd.exe" }</code>
                                { " and hit "}
                                <kbd>{ "Enter" }</kbd>
                                { ". This should open a command prompt." }
                            </li>
                            <li>
                                { "Type " }
                                <code>{ "wmic cpu get name" }</code>
                                { "in the prompt and hit" }
                                <kbd>{ "Enter" }</kbd>
                                { ". The output should look something like this." }
                                <code>
                                    <p>{ "Name" }</p>
                                    <p style="margin-top: -1rem">
                                        { "Intel(R) Core(TM) i9-10900K CPU @ 3.70GHz" }
                                    </p>
                                </code>
                                </li>
                            <li>
                                { "Your CPU model is the second line of the output. In this case: " }
                                <code>{ "Intel(R) Core(TM) i9-10900K CPU @ 3.70GHz" }</code>
                            </li>
                        </ol>
                    </AccordionCollapse>
                </AccordionItem>
                <AccordionItem>
                    <AccordionHeader id="headingTwo" data_bs_target="#collapseTwo">
                        { "Linux" }
                    </AccordionHeader>
                    <AccordionCollapse data_bs_parent="#modelMethodsAccordion" id="collapseTwo">
                        <ol>
                            <li>{"Open a terminal."}</li>
                            <li>
                                {"Type "}
                                <code>{"grep 'model name' /proc/cpuinfo | uniq"}</code>
                                {" and hit "}
                                <kbd>{"Enter"}</kbd>
                                {"."}
                                {" The output should look something like this:"}
                                <code>
                                    <p>{"model name: Intel(R) Core(TM) i9-10900K CPU @ 3.70GHz"}</p>
                                </code>
                            </li>
                            <li>
                                {"Your CPU model is the second part of the output. In this case: "}
                                <code>{" Intel(R) Core(TM) i9-10900K CPU @ 3.70GHz "}</code>
                            </li>
                        </ol>
                    </AccordionCollapse>
                </AccordionItem>
                <AccordionItem>
                    <AccordionHeader id="headingThree" data_bs_target="#collapseThree">
                        { "MacOS" }
                    </AccordionHeader>
                    <AccordionCollapse data_bs_parent="#modelMethodsAccordion" id="collapseThree">
                        <ol>
                            <li>{"Open a terminal."}</li>
                            <li>
                                {"Type "}
                                <code>{"sysctl machdep.cpu.brand_string"}</code>
                                {" and hit "}
                                <kbd>{"Enter"}</kbd>
                                {"."}
                                {" The output should look something like this:"}
                                <code>
                                    <p>
                                        {"machdep.cpu.brand_string: Intel(R) Core(TM) i9-10900K CPU @ 3.70GHz"}
                                    </p>
                                </code>
                            </li>
                            <li>
                                {"Your CPU model is the second part of the output. In this case: "}
                                <code>{" Intel(R) Core(TM) i9-10900K CPU @ 3.70GHz "}</code>
                            </li>
                        </ol>
                    </AccordionCollapse>
                </AccordionItem>
                <AccordionItem>
                    <AccordionHeader id="headingFour" data_bs_target="#collapseFour">
                        { "Android" }
                    </AccordionHeader>
                    <AccordionCollapse data_bs_parent="#modelMethodsAccordion" id="collapseFour">
                        { "Unfortunately, there is no direct way of determining your CPU model on
                        Android. Please use your favorite search engine to determine your CPU
                        model or simply state the name of your mobile device." }
                    </AccordionCollapse>
                </AccordionItem>
            </Accordion>
            <div style="padding-left: 2rem; padding-right: 2rem; padding-top: 2rem">
                <label for="model" class="form-label">
                    <strong>{ "CPU model" }</strong>
                </label>
                <input
                    id="model"
                    name="model"
                    type="text"
                    placeholder="Please enter your CPU model here"
                    aria-label="CPU model"
                    class="form-control"
                    value={model_input.clone()}
                    oninput={ctx.link().callback(|e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        AppRootMessage::ChangeModel(input.value())
                    })}
                    disabled={input_disabled}
                    required=true
                />
            </div>
        </>
    }
}

fn render_benchmark_instructions(ctx: &Context<AppRoot>, button_disabled: bool) -> Html {
    html! {
        <>
            <h5 style="padding-left: 2rem; padding-right: 2rem; padding-top: 3rem">
                { "Step 2 - Running our benchmarks." }
            </h5>
            <p style="padding-left: 2rem; padding-right: 2rem">
                {"Please do "}
                <strong>{"not"}</strong>
                {" do anything else on your computer while running our benchmarks.
                To ensure that you leave the tab open, you will have to press the button
                at the bottom at least every 30 seconds. Press the START-button to start."}
            </p>
            <div style="display: flex; justify-content: center; margin: 3rem">
                <button
                    id="startButton"
                    class="btn btn-primary btn-lg"
                    style="width: 6.5rem"
                    type="button"
                    onclick={ctx.link().callback(|_| { AppRootMessage::StartBenchmarks })}
                    disabled={button_disabled}
                >
                    { "Run tests" }
                </button>
            </div>
        </>
    }
}

fn render_progress_bar(current_progress: f32, status_label: String) -> Html {
    html! {
        <>
            <Container>
                <Container size={ContainerSize::Large}>
                    <p style="text-align: center">{ "Total progress:" }</p>
                    <div class="progress">
                        <div
                            id="totalBar"
                            class="progress-bar"
                            role="progressbar"
                            style={format!("width: {}%", current_progress)}
                            aria-valuenow="0"
                            aria-valuemin="0"
                            aria-valuemax="100">
                        </div>
                    </div>
                </Container>
            </Container>
            <p>{status_label}</p>
        </>
    }
}

pub fn render_footer() -> Html {
    html! {
        <>
        <Container size={ContainerSize::Large}>
            <div style="padding-top: 8rem">
            </div>
        </Container>
        <footer class="text-center text-white fixed-bottom bg-dark">
            <div class="text-center p-3">
                <p>{" Hacking Lab team @ TUDelft 2023 "}</p>
            </div>
        </footer>
        </>
    }
}
