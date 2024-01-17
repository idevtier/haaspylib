use core::fmt;
use serde::Deserialize;

use haaslib::model::CustomReport;
use pyo3::{prelude::*, types::PyIterator};

type Result<T = ()> = std::result::Result<T, Error>;

struct Error {
    inner: haaslib::Error,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl From<Error> for PyErr {
    fn from(value: Error) -> Self {
        Self::new::<PyAny, _>(value.to_string())
    }
}

impl From<haaslib::Error> for Error {
    fn from(value: haaslib::Error) -> Self {
        Self { inner: value }
    }
}

#[pyclass]
struct Executor {
    inner: haaslib::api::ReqwestExecutor<haaslib::api::Authenticated>,
}

#[pymethods]
impl Executor {
    #[new]
    fn new(address: &str, port: u16, protocol: &str, email: &str, password: &str) -> Self {
        Self {
            inner: haaslib::api::ReqwestExecutor::new(address, port, protocol)
                .authenticate(&email.to_string(), &password.to_string())
                .unwrap(),
        }
    }
}

#[pyclass]
struct CloudMarket {
    inner: haaslib::model::CloudMarket,
}

#[pymethods]
impl CloudMarket {
    #[getter]
    fn price_source(&self) -> String {
        self.inner.price_source.to_string()
    }

    #[getter]
    fn primary(&self) -> String {
        self.inner.primary.to_string()
    }

    #[getter]
    fn secondary(&self) -> String {
        self.inner.secondary.to_string()
    }

    fn as_market_tag(&self) -> String {
        self.inner.as_market_tag()
    }
}

#[pyclass]
struct HaasScriptItemWithDependencies {
    inner: haaslib::model::HaasScriptItemWithDependencies,
}

#[pymethods]
impl HaasScriptItemWithDependencies {
    #[getter]
    fn id(&self) -> &str {
        &self.inner.script_id
    }
}

#[pyclass]
struct UserAccount {
    inner: haaslib::model::UserAccount,
}

#[pymethods]
impl UserAccount {
    #[getter]
    fn id(&self) -> &str {
        &self.inner.account_id
    }
}

#[pyclass]
#[derive(Clone, Debug)]
struct CreateLabRequest {
    pub script_id: String,
    pub name: String,
    pub account_id: String,
    pub market: String,
    pub interval: u32,
}

#[pymethods]
impl CreateLabRequest {
    #[new]
    fn new(
        script_id: String,
        name: String,
        account_id: String,
        market: String,
        interval: u32,
    ) -> Self {
        Self {
            script_id,
            name,
            account_id,
            market,
            interval,
        }
    }

    fn __str__(&self) -> String {
        format!("{:?}", self)
    }

    fn __repr__(&self) -> String {
        self.__str__()
    }
}

#[pyclass]
#[derive(Clone)]
struct UserLabDetails {
    inner: haaslib::model::UserLabDetails,
}

#[pyclass]
#[derive(Clone)]
struct StartLabExecutionRequest {
    inner: haaslib::model::StartLabExecutionRequest<'static>,
}

#[pyclass]
#[derive(Clone)]
struct PaginatedResponse {
    items: Vec<Py<PyAny>>,
    next_page_id: i64,
}

#[pyclass]
#[derive(Clone, Deserialize)]
struct PyCustomReport {
    report: String,
}

impl CustomReport for PyCustomReport {}

#[pyclass]
#[derive(Clone)]
struct UserLabBacktestResult {
    inner: haaslib::model::UserLabBacktestResult<PyCustomReport>,
}

#[pyfunction]
fn get_all_markets(executor: &Executor) -> Result<Vec<CloudMarket>> {
    haaslib::api::get_all_markets(&executor.inner)
        .map_err(Error::from)
        .map(|markets| {
            markets
                .into_iter()
                .map(|m| CloudMarket { inner: m })
                .collect::<Vec<_>>()
        })
}

#[pyfunction]
fn get_all_markets_by_price_source(
    executor: &Executor,
    price_source: &str,
) -> Result<Vec<CloudMarket>> {
    haaslib::api::get_all_markets_by_pricesource(&executor.inner, price_source)
        .map_err(Error::from)
        .map(|markets| {
            markets
                .into_iter()
                .map(|m| CloudMarket { inner: m })
                .collect::<Vec<_>>()
        })
}

#[pyfunction]
fn get_all_script_items(executor: &Executor) -> Result<Vec<HaasScriptItemWithDependencies>> {
    haaslib::api::get_all_script_items(&executor.inner)
        .map_err(Error::from)
        .map(|scripts| {
            scripts
                .into_iter()
                .map(|inner| HaasScriptItemWithDependencies { inner })
                .collect::<Vec<_>>()
        })
}

#[pyfunction]
fn get_accounts(executor: &Executor) -> Result<Vec<UserAccount>> {
    haaslib::api::get_accounts(&executor.inner)
        .map_err(Error::from)
        .map(|accounts| {
            accounts
                .into_iter()
                .map(|acc| UserAccount { inner: acc })
                .collect::<Vec<_>>()
        })
}

#[pyfunction]
fn create_lab(executor: &Executor, req: CreateLabRequest) -> Result<UserLabDetails> {
    let req = haaslib::model::CreateLabRequest {
        script_id: &req.script_id,
        market: &req.market,
        account_id: &req.account_id,
        name: &req.name,
        interval: req.interval,
        style: haaslib::model::HaasChartPricePlotStyle::CandleStick,
    };

    haaslib::api::create_lab(&executor.inner, req)
        .map_err(Error::from)
        .map(|inner| UserLabDetails { inner })
}

#[pyfunction]
fn start_lab_execution(
    executor: &Executor,
    req: StartLabExecutionRequest,
) -> Result<UserLabDetails> {
    haaslib::api::start_lab_execution(&executor.inner, req.inner)
        .map_err(Error::from)
        .map(|inner| UserLabDetails { inner })
}

#[pyfunction]
fn get_lab_details(executor: &Executor, lab_id: &str) -> Result<UserLabDetails> {
    haaslib::api::get_lab_details(&executor.inner, lab_id)
        .map_err(Error::from)
        .map(|inner| UserLabDetails { inner })
}

#[pyfunction]
fn update_lab_details(executor: &Executor, details: &UserLabDetails) -> Result<UserLabDetails> {
    haaslib::api::update_lab_details(&executor.inner, &details.inner)
        .map_err(Error::from)
        .map(|inner| UserLabDetails { inner })
}

#[pyfunction]
fn update_multiple_labs_details(
    executor: &Executor,
    details: &PyIterator,
) -> Result<Vec<UserLabDetails>> {
    let details = details
        .into_iter()
        .map(|i| {
            i.and_then(PyAny::extract::<UserLabDetails>)
                .map(|d| d.inner)
                .unwrap()
        })
        .collect::<Vec<_>>();

    haaslib::api::update_multiple_lab_details(&executor.inner, &details)
        .map_err(Error::from)
        .map(|details| {
            details
                .into_iter()
                .map(|inner| UserLabDetails { inner })
                .collect::<Vec<_>>()
        })
}

#[pyfunction]
fn get_backtest_result(
    py: Python<'_>,
    executor: &Executor,
    lab_id: &str,
    next_page_id: u64,
    page_length: u32,
) -> Result<PaginatedResponse> {
    haaslib::api::get_backtest_result(&executor.inner, lab_id, next_page_id, page_length)
        .map_err(Error::from)
        .map(|resp| PaginatedResponse {
            items: resp
                .items
                .into_iter()
                .map(|i| UserLabBacktestResult { inner: i }.into_py(py))
                .collect::<Vec<_>>(),
            next_page_id: resp.next_page_id,
        })
}

pub fn register(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    let child_module = PyModule::new(py, "api")?;

    child_module.add_class::<Executor>()?;
    child_module.add_class::<CloudMarket>()?;
    child_module.add_class::<HaasScriptItemWithDependencies>()?;
    child_module.add_class::<UserAccount>()?;
    child_module.add_class::<UserLabDetails>()?;
    child_module.add_class::<CreateLabRequest>()?;
    child_module.add_class::<PaginatedResponse>()?;
    child_module.add_class::<UserLabBacktestResult>()?;

    child_module.add_function(wrap_pyfunction!(get_all_markets, m)?)?;
    child_module.add_function(wrap_pyfunction!(get_all_markets_by_price_source, m)?)?;
    child_module.add_function(wrap_pyfunction!(get_all_script_items, m)?)?;
    child_module.add_function(wrap_pyfunction!(get_accounts, m)?)?;
    child_module.add_function(wrap_pyfunction!(create_lab, m)?)?;
    child_module.add_function(wrap_pyfunction!(start_lab_execution, m)?)?;
    child_module.add_function(wrap_pyfunction!(get_lab_details, m)?)?;
    child_module.add_function(wrap_pyfunction!(update_lab_details, m)?)?;
    child_module.add_function(wrap_pyfunction!(update_multiple_labs_details, m)?)?;
    child_module.add_function(wrap_pyfunction!(get_backtest_result, m)?)?;

    m.add_submodule(child_module)?;

    Ok(())
}
