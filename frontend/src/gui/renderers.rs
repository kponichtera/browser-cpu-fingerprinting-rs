use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::{html, Context, Html};
use yew_bootstrap::component::*;

use crate::gui::app_root::AppRootMessage;
use crate::gui::app_root::{AppRoot, ExperimentResult};
use crate::gui::components::*;

pub fn render_main_container(
    model_input: &str,
    input_disabled: bool,
    ctx: &Context<AppRoot>,
    button_disabled: bool,
    finished_benchmarks: usize,
    total_benchmarks: usize,
    status_label: &str,
    experiment_result: &ExperimentResult,
) -> Html {
    html! {
        <Container>
            {render_header()}
            {render_cpu_model_instructions(model_input.to_string(), input_disabled, ctx)}
            {render_benchmark_instructions()}
            {render_start_button(ctx, button_disabled)}
            {render_progress_bar(experiment_result, finished_benchmarks, total_benchmarks, status_label.to_string())}
            {render_next_experiment_button(experiment_result)}
        </Container>
    }
}

fn render_header() -> Html {
    html! {
        <>
            <h1 style="text-align: center; padding-top: 3rem">
                { "Browser CPU fingerprinting" }
            </h1>
            <p style="margin: 2rem">
                { "This site will run a few JavaScript benchmarks to gather information
                about your CPU. This results of these benchmarks will then be uploaded
                to our server, where they are then stored in a database.
                We do not collect or store any personal data. Our project is based on the research
                of CISPA Helmholtz Center for Information Security in Saarbrucken, Saarland, Germany.
                To access the original paper, click " }
                <a href="https://publications.cispa.saarland/3745/1/paper.pdf" target="_blank">{ "here" }</a>
                { "." }
            </p>
            <p style="padding-left: 2rem; padding-right: 2rem">
                { "Our benchmarks are designed for the latest versions of " }
                <strong>{ "Firefox" }</strong>
                { " and " }
                <strong>{ "Chrome" }</strong>
                { "-based browsers (e.g. Google Chrome, Chromium, Microsoft Edge etc.). " }
                { "Safari and iOS devices are not supported." }
            </p>
        </>
    }
}

pub fn render_cpu_model_instructions(
    model_input: String,
    input_disabled: bool,
    ctx: &Context<AppRoot>,
) -> Html {
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
                                { " in the prompt and hit " }
                                <kbd>{ "Enter" }</kbd>
                                { ". The output should look something like this." }
                                <code>
                                    <p>{ "Name" }</p>
                                    <p style="margin-top: -1rem">
                                        { "Intel(R) Core(TM) i3-10900K CPU @ 3.10GHz" }
                                    </p>
                                </code>
                                </li>
                            <li>
                                { "Your CPU model is the second line of the output. In this case: " }
                                <code>{ "Intel(R) Core(TM) i3-10900K CPU @ 3.10GHz" }</code>
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
                                    <p>{"model name: Intel(R) Core(TM) i3-10900K CPU @ 3.10GHz"}</p>
                                </code>
                            </li>
                            <li>
                                {"Your CPU model is the second part of the output. In this case: "}
                                <code>{" Intel(R) Core(TM) i3-10900K CPU @ 3.10GHz "}</code>
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
                                        {"machdep.cpu.brand_string: Intel(R) Core(TM) i3-10900K CPU @ 3.10GHz"}
                                    </p>
                                </code>
                            </li>
                            <li>
                                {"Your CPU model is the second part of the output. In this case: "}
                                <code>{" Intel(R) Core(TM) i3-10900K CPU @ 3.10GHz "}</code>
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

fn render_benchmark_instructions() -> Html {
    html! {
        <>
            <h5 style="padding-left: 2rem; padding-right: 2rem; padding-top: 3rem">
                { "Step 2 - Running our benchmarks." }
            </h5>
            <p style="padding-left: 2rem; padding-right: 2rem">
                {"Please do "}
                <strong>{"not"}</strong>
                {" do anything else on your computer while running our benchmarks,
                so that the benchmark results are the most accurate.
                Press the button below to start. Once the experiment finishes successfully,
                click the button that appears to proceed to second experiment."}
            </p>
        </>
    }
}

fn render_start_button(ctx: &Context<AppRoot>, button_disabled: bool) -> Html {
    html! {
        <div style="display: flex; justify-content: center; margin: 3rem">
            <button
                id="startButton"
                class="btn btn-primary btn-lg"
                style="width: 6.5rem"
                type="button"
                onclick={ctx.link().callback(|_| { AppRootMessage::StartBenchmarks })}
                disabled={button_disabled}
            >
                { "START" }
            </button>
        </div>
    }
}

fn render_next_experiment_button(experiment_result: &ExperimentResult) -> Html {
    let button_visibility = match experiment_result {
        ExperimentResult::Success => "visible",
        _ => "hidden",
    };
    html! {
        <div style="display: flex; justify-content: center; margin: 3rem">
            <a
                id="nextExperimentButton"
                href="https://benchmark2.ponichtera.dev/start/"
                class="btn btn-success btn-lg"
                style={ format!("width: 12rem; visibility: {}", button_visibility) }
            >
                { "Next experiment" }
            </a>
        </div>
    }
}

fn render_progress_bar(
    experiment_result: &ExperimentResult,
    finished_benchmarks: usize,
    total_benchmarks: usize,
    status_label: String,
) -> Html {
    let progress = finished_benchmarks as f32 / total_benchmarks as f32 * 100.0;

    let progress_bar_classes = match experiment_result {
        ExperimentResult::Running => "progress-bar-striped progress-bar-animated",
        ExperimentResult::Success => "bg-success",
        ExperimentResult::Error => "bg-danger",
        _ => "",
    };

    let progress_bar_visibility = match experiment_result {
        ExperimentResult::NotStarted => "hidden",
        _ => "visible",
    };

    html! {
        <>
            <Container>
                <Container size={ContainerSize::Large}>
                    <div class="progress" style={format!("height: 2rem; visibility: {}", progress_bar_visibility)}>
                        <div
                            id="totalBar"
                            class={format!("progress-bar benchmark-progress-bar {}", progress_bar_classes)}
                            role="progressbar"
                            style={format!("width: {}%", progress)}
                            aria-valuenow="0"
                            aria-valuemin="0"
                            aria-valuemax="100">
                            { status_label }
                        </div>
                    </div>
                </Container>
            </Container>
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
                <p>{" D. Plămădeală, J. van Vliet, J. Naucke, C. Xu, K. Ponichtera @ TU Delft 2023 "}</p>
            </div>
        </footer>
        </>
    }
}
