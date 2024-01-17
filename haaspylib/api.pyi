from collections.abc import Iterable
from typing import Any

class Executor:
    def __init__(
        self, address: str, port: int, email: str, password: str, protocol: str
    ): ...

class CloudMarket:
    price_source: str
    primary: str
    secondary: str

    def as_market_tag(self) -> str: ...

class HaasScriptItemWithDependencies:
    id: str

class UserAccount:
    id: str

class CreateLabRequest:
    name: str
    script_id: str
    account_id: str
    market: str
    interval: int

    def __init__(
        self, name: str, script_id: str, account_id: str, market: str, interval: int
    ) -> None: ...

class UserLabDetails:
    pass

class StartLabExecutionRequest:
    pass

class PaginatedResponse:
    items: list[Any]
    next_page_id: int

def get_all_markets(executor: Executor) -> list[CloudMarket]: ...
def get_all_script_items(
    executor: Executor,
) -> list[HaasScriptItemWithDependencies]: ...
def get_all_markets_by_price_source(
    executor: Executor, price_source: str
) -> list[CloudMarket]: ...
def get_accounts(executor: Executor) -> list[UserAccount]: ...
def create_lab(executor: Executor, req: CreateLabRequest) -> UserLabDetails: ...
def start_lab_execution(
    executor: Executor, req: StartLabExecutionRequest
) -> UserLabDetails: ...
def get_lab_details(executor: Executor, lab_id: str) -> UserLabDetails: ...
def update_lab_details(
    executor: Executor, details: UserLabDetails
) -> UserLabDetails: ...
def update_multiple_labs_details(
    executor: Executor, details: Iterable[UserLabDetails]
) -> list[UserLabDetails]: ...
def get_backtest_result(
    executor: Executor, lab_id: str, next_page_id: int, page_length: int
) -> PaginatedResponse: ...
