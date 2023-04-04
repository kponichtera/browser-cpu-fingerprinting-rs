use std::collections::VecDeque;
use std::ops::Deref;
use std::rc::Rc;

use gloo_net::http::Request;
use serde_json::value::Value;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};
use yew_bootstrap::component::*;
use yew_bootstrap::util::*;

use common::dto::result::ResultDTO;

use crate::profilers::cache_associativity::*;
use crate::profilers::cache_size::*;
use crate::profilers::load_buffer_size::*;
use crate::profilers::memory_latencies::*;
use crate::profilers::multi_core_performance::*;
use crate::profilers::page_size::*;
use crate::profilers::prefetcher::*;
use crate::profilers::Profiler;
use crate::profilers::single_core_performance::*;
use crate::profilers::timer_precision::*;
use crate::profilers::tlb_size::*;
use crate::worker::{BenchmarkInput, BenchmarkResult, BenchmarkType, BenchmarkWorker};

pub enum AppRootMessage {
    ChangeModel(String),
    StartBenchmarks,
    BenchmarkComplete(BenchmarkResult),
    BenchmarksFinished(String),
}

pub struct AppRoot {
    bridge: Box<dyn Bridge<BenchmarkWorker>>,

    model_input: String,
    status_label: String,
    button_disabled: bool,
    input_disabled: bool,

    benchmark_results: Vec<BenchmarkResult>,
    remaining_benchmarks: VecDeque<BenchmarkType>,
}


// all of these these should ideally be moved somewhere else
#[derive(PartialEq, Properties)]
struct AccordionProps {
    style: Option<String>,
    id: Option<String>,
    children: Children,
}

#[function_component]
fn Accordion(props: &AccordionProps) -> Html {
    html! {
        <div class="accordion" style={props.style.clone()} id={props.id.clone()}>
            { props.children.clone() }
        </div>
    }
}

#[function_component]
fn AccordionItem(props: &AccordionProps) -> Html {
    html! {
        <div class="accordion-item" style={props.style.clone()} id={props.id.clone()}>
            { props.children.clone() }
        </div>
    }
}

#[derive(PartialEq, Properties)]
struct AccordionButtonProps {
    style: Option<String>,
    id: Option<String>,
    children: Children,
    data_bs_target: String,
}

#[function_component]
fn AccordionButton(props: &AccordionButtonProps) -> Html {
    html! {
        <button class="accordion-button collapsed" type="button" data-bs-toggle="collapse" data-bs-target={props.data_bs_target.clone()} aria-expanded="false" aria-controls={props.id.clone()}>
            { props.children.clone() }
        </button>
    }
}

#[derive(PartialEq, Properties)]
struct AccordionHeaderProps {
    style: Option<String>,
    id: Option<String>,
    children: Children,
    data_bs_target: String,
}

#[function_component]
fn AccordionHeader(props: &AccordionHeaderProps) -> Html {
    html! {
        <h2 class="accordion-header" style={props.style.clone()} id={props.id.clone()}>
            <AccordionButton id={props.id.clone()} style={props.style.clone()} data_bs_target={props.data_bs_target.clone()}>
                { props.children.clone() }
            </AccordionButton>
        </h2>
    }
}

#[derive(PartialEq, Properties)]
struct AccordionCollapseProps {
    style: Option<String>,
    id: Option<String>,
    children: Children,
    data_bs_parent: String,
}

#[function_component]
fn AccordionCollapse(props: &AccordionCollapseProps) -> Html {
    html! {
        <div class="accordion-collapse collapse" id={props.id.clone()} aria-labelledby={props.id.clone()} data-bs-parent={props.data_bs_parent.clone()}>
            <div class="accordion-body">
                { props.children.clone() }
            </div>
        </div>
    }
}

impl Component for AppRoot {
    type Message = AppRootMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        let worker_result_callback =
            move |result| link.send_message(AppRootMessage::BenchmarkComplete(result));

        AppRoot {
            bridge: BenchmarkWorker::bridge(Rc::new(worker_result_callback)),
            model_input: String::default(),
            status_label: String::default(),
            button_disabled: false,
            input_disabled: false,
            benchmark_results: Vec::new(),
            remaining_benchmarks: VecDeque::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppRootMessage::ChangeModel(new_model) => {
                self.model_input = new_model;
                true
            }
            AppRootMessage::StartBenchmarks => {
                self.start_benchmarks();
                true
            }
            AppRootMessage::BenchmarkComplete(result) => {
                self.handle_benchmark_complete(ctx, result);
                true
            }
            AppRootMessage::BenchmarksFinished(status) => {
                self.handle_benchmarks_finished(status);
                true
            }
        }
    }


    fn view(&self, ctx: &Context<Self>) -> Html {
        let button_disabled = self.button_disabled || self.model_input.is_empty();

        html! {
        <>
            {include_cdn()}
            <Container>
                <Container size={ContainerSize::Large}>
                    <h1 style={"text-align: center"}>
                        { "RUST WASM CPU fingerprinting" }
                    </h1>
                    <p style={"margin: 2rem"}>
                        { "This site will run a few JavaScript benchmarks to gather information about your CPU. This results of these benchmarks will then be uploaded to our server, where they are then stored in a database. Please click " }
                        <code>{ "Continue" }</code>
                        { " and follow the short instructions on the next page. For more information check out the FAQ at the bottom." }
                    </p>
                    <p style="padding-left: 2rem; padding-right: 2rem">
                        { "Our benchmarks are designed for the latest versions of " }
                        <strong>{ "Firefox" }</strong>
                        { " and " }
                        <strong>{ "Chrome" }</strong>
                        { "-based browsers (e.g. Google Chrome, Chromium, Microsoft Edge etc.). Most importantly Safari and iOS devices are not supported." }
                    </p>
                    <h5 style="padding-left: 2rem; padding-right: 2rem; padding-top: 3rem">
                        { "Step 1 - Determining your CPU model." }
                    </h5>
                    <p style="padding-left: 2rem; padding-right: 2rem">
                        { "Please fill out the text field below with your CPU model. To keep our database consistent please use the following method to determine your CPU model." }
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
                            value={self.model_input.clone()}
                            oninput={ctx.link().callback(|e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                AppRootMessage::ChangeModel(input.value())
                            })}
                            disabled={self.input_disabled}
                            required=true
                        />
                    </div>
                    <h5 style="padding-left: 2rem; padding-right: 2rem; padding-top: 3rem">
                        { "Step 2 - Running our benchmarks." }
                    </h5>
                    <p style="padding-left: 2rem; padding-right: 2rem">
                        {"Please do "}
                        <strong>{"not"}</strong>
                        {" do anything else on your computer while running our benchmarks. To ensure that you leave the tab open, you will have to press the button at the bottom at least every 30 seconds. Press the START-button to start."}
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
                    <Container>
                        <Container size={ContainerSize::Large}>
                            <p style="text-align: center">{ "Total progress:" }</p>
                            <div class="progress">
                                <div id="totalBar" class="progress-bar" role="progressbar" style="width: 0%" aria-valuenow="0" aria-valuemin="0" aria-valuemax="100"></div>
                            </div>
                        </Container>
                    </Container>
                    <Container>
                        <Container size={ContainerSize::Large}>
                            <p style="text-align: center">{ "Current benchmark:" }</p>
                            <div class="progress">
                                <div id="mainBar" class="progress-bar" role="progressbar" style="width: 0%" aria-valuenow="0" aria-valuemin="0" aria-valuemax="100"></div>
                            </div>
                        </Container>
                    </Container>
                    <p>{self.status_label.clone()}</p>
                </Container>
            </Container>
            <Container size={ContainerSize::Large}>
                <div style="padding-top: 8rem">
                </div>
            </Container>
            {include_cdn_js()}
            <footer class="text-center text-white fixed-bottom bg-dark">
                <div class="text-center p-3">
                    <p>{" Hacking Lab team @ TUDelft 2023 "}</p>
                </div>
            </footer>
        </>
        }
    }
}

impl AppRoot {
    fn start_benchmarks(&mut self) {
        self.button_disabled = true;
        self.input_disabled = true;
        self.benchmark_results = vec![];

        self.remaining_benchmarks = VecDeque::from(vec![
            // TODO: Add remaining benchmarks
            BenchmarkType::PageSize,
            BenchmarkType::CacheSize,
        ]);

        // Start with first benchmark
        let benchmark = self
            .remaining_benchmarks
            .pop_front()
            .expect("No benchmarks specified");
        self.status_label = benchmark.to_string();
        self.bridge.send(BenchmarkInput { benchmark });
    }

    fn handle_benchmark_complete(&mut self, ctx: &Context<Self>, result: BenchmarkResult) {
        self.benchmark_results.push(result);

        let next_benchmark = self.remaining_benchmarks.pop_front();
        match next_benchmark {
            Some(benchmark) => {
                // Run next benchmark
                self.status_label = benchmark.to_string();
                self.bridge.send(BenchmarkInput { benchmark });
            }
            None => {
                // No more benchmarks - send results to backend
                self.send_result(ctx);
            }
        }
    }

    fn send_result(&self, ctx: &Context<Self>) {
        let (results, times) = self.parse_results();

        let result = ResultDTO {
            model: self.model_input.clone(),
            user_agent: get_user_agent().unwrap_or_else(|| "unknown".to_string()),
            benchmark_results: results,
            times,
        };

        let link = ctx.link().clone();

        wasm_bindgen_futures::spawn_local(async move {
            let status = Request::post("/api/result/upload")
                .json(&result)
                .unwrap()
                .send()
                .await
                .unwrap()
                .status_text();

            link.send_message(AppRootMessage::BenchmarksFinished(status));
        });
    }

    fn handle_benchmarks_finished(&mut self, status: String) {
        self.button_disabled = false;
        self.input_disabled = false;
        self.status_label = status;
    }

    fn parse_results(&self) -> (Vec<Value>, Vec<f32>) {
        let mut results = vec![];
        let mut times = vec![];

        for result in self.benchmark_results.iter() {
            let value = serde_json::from_str::<Value>(result.result_json.clone().as_str()).unwrap();

            // TODO: Cloning the whole result JSON is not very optimal
            results.push(value);
            times.push(result.time);
        }

        (results, times)
    }
}

fn get_user_agent() -> Option<String> {
    let window = web_sys::window().expect("Missing window");
    let user_agent = window.navigator().user_agent();
    match user_agent {
        Ok(user_agent) => Some(user_agent),
        Err(_) => None,
    }
}

/// TODO: For removal once benchmarks are fully handled by dedicated worker(s)
fn run_profilers<T>(profiler_prehook: T) -> (Vec<Value>, Vec<f32>)
    where
        T: FnOnce(&dyn Profiler) + Copy,
{
    let profilers: Vec<Box<dyn Profiler>> = vec![
        Box::new(PageSizeProfiler {}),
        Box::new(PrefetcherProfiler {}),
        Box::new(CacheAssociativityProfiler {}),
        Box::new(CacheSizeProfiler {}),
        Box::new(TlbSizeProfiler {}),
        Box::new(TimerPrecisionProfiler {}),
        Box::new(MemoryLatenciesProfiler {}),
        Box::new(LoadBufferSizeProfiler {}),
        Box::new(SingleCorePerformanceProfiler {}),
        Box::new(MultiCorePerformanceProfiler {}),
    ];

    let mut results = vec![];
    let mut times = vec![];

    for profiler in profilers {
        profiler_prehook(profiler.deref());
        let result = profiler.run();
        results.push(result.0);
        times.push(result.1);
    }

    (results, times)
}
