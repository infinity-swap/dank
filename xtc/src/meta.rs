use crate::common_types::Metadata;
use crate::fee::compute_fee;
use crate::stats::StatsData;
use ic_kit::macros::*;
use ic_kit::{
    candid::{CandidType, Nat},
    ic,
};

#[derive(CandidType)]
pub struct TokenMetaData<'a> {
    pub name: &'a str,
    pub symbol: &'a str,
    pub decimal: u8,
    pub features: Vec<&'a str>,
}

#[query]
pub fn meta() -> TokenMetaData<'static> {
    TokenMetaData {
        name: "Cycles",
        symbol: "XTC",
        decimal: 12,
        features: vec!["history"],
    }
}

#[query(name = "getMetadata")]
pub fn get_metadata() -> Metadata<'static> {
    Metadata {
        decimals: 12,
        fee: Nat::from(compute_fee(0)),
        logo: "PHN2ZyB3aWR0aD0iMTc2IiBoZWlnaHQ9IjE3NiIgdmlld0JveD0iMCAwIDE3NiAxNzYiIGZpbGw9Im5vbmUiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+CjxyZWN0IHg9IjIuNSIgeT0iMi41IiB3aWR0aD0iMTcxIiBoZWlnaHQ9IjE3MSIgcng9Ijg1LjUiIGZpbGw9ImJsYWNrIiBzdHJva2U9ImJsYWNrIiBzdHJva2Utd2lkdGg9IjUiLz4KPHBhdGggZD0iTTExNS41MjUgMTIxLjgxTDExMy42MDMgMTEwLjU3NUMxMDkuMzc3IDExNS42NTcgMTAzLjc3NCAxMTkuNDEyIDk3LjQ2NTUgMTIxLjM5MkM4NS40NjU4IDEyNS4xNzUgNzIuOTQzNCAxMjIuMDcgNjQuMTU2OCAxMTQuMzA4QzYzLjcyODUgMTEzLjkyNyA2My4zMDkgMTEzLjUzNiA2Mi44OTgzIDExMy4xMzVDNjIuNDg3NiAxMTIuNzM0IDYyLjA5MjkgMTEyLjMzIDYxLjcxNDQgMTExLjkyMUw2Ny40ODk5IDEwOC41NzdMNDYuNTY3MSAxMDAuODRMNDIuODcyMSAxMjIuODMyTDQ4LjA3IDExOS44MjJDNDguNDE5MSAxMjAuMjU0IDQ4Ljc2ODMgMTIwLjY4NiA0OS4xMzEgMTIxLjEwN0M0OS40OTM3IDEyMS41MjkgNDkuODcyMiAxMjEuOTQ4IDUwLjI1MjMgMTIyLjM1OUM2Mi45OTYzIDEzNi4xNjUgODMuMDE5NSAxNDIuMTk2IDEwMi4xMzMgMTM2LjE3MkMxMTIuNjYgMTMyLjg2OSAxMjEuNzkxIDEyNi4xNjcgMTI4LjA5NiAxMTcuMTE5TDExNS41MjUgMTIxLjgxWiIgZmlsbD0id2hpdGUiLz4KPHBhdGggZD0iTTQzLjk3OTIgOTYuMzc2M0w1NC42MjY5IDEwMC4zMTJDNTQuNDUwNCA5OS44MjkyIDU0LjI3OTIgOTkuMzQwOSA1NC4xMjMxIDk4Ljg0NzNDNDguNDM5NiA4MC44NDM1IDU4LjI4NTcgNjEuNjY1NSA3Ni4xNDI0IDU1LjcxNzVDNzYuMzIyNiA1NS42NTcyIDc2LjQ5OTggNTUuNTg5NCA3Ni42ODIzIDU1LjUzMTRDNzcuMDQ5NiA1NS40MTYxIDc3LjQxNzYgNTUuMzE2NiA3Ny43ODU1IDU1LjIxMzRDNzguMzMzIDU1LjA2MjYgNzguODgyNyA1NC45MjE3IDc5LjQzMDIgNTQuNzk3NEw3OS40NDIzIDYxLjQyODlMOTYuNjA2IDQ3LjE4NjJMNzkuMzkzMyAzM0w3OS40MDM4IDM5LjAxOTZDNzguODU2NCAzOS4xMDYzIDc4LjMwNTkgMzkuMjE0IDc3Ljc1ODQgMzkuMzIxMUM3Ny4yMTA5IDM5LjQyODEgNzYuNjYyIDM5LjU0MDMgNzYuMTEzNyAzOS42NjM5Qzc0Ljc0MzYgMzkuOTcyOSA3My4zNzY0IDQwLjMyNjMgNzIuMDEzOCA0MC43NTU5QzQ1LjY0NjUgNDkuMDY0MSAzMS4wMTUgNzcuMTYgMzkuMzMyNSAxMDMuNTExQzQwLjAwMTUgMTA1LjYxNiA0MC44MDc1IDEwNy42NzUgNDEuNzQ1NiAxMDkuNjc1TDQzLjk3OTIgOTYuMzc2M1oiIGZpbGw9IndoaXRlIi8+CjxwYXRoIGQ9Ik0xMzQuMzM2IDEwNC45NkMxMzQuNTIxIDEwNC40MzIgMTM0LjY4NyAxMDMuODk3IDEzNC44NTUgMTAzLjM2MUMxMzcuOTE3IDkzLjYxMSAxMzcuOTAzIDgzLjE1NTggMTM0LjgxNSA3My40MTQyQzEyOC41MzQgNTMuNTE1MSAxMTAuOTU4IDQwLjMxIDkxLjM3MyAzOC42MTUyTDEwMS43NjUgNDcuMTg0Mkw5My4wMDY0IDU0LjQ1MDNDMTA1LjMwNCA1Ni42MDI1IDExNi4wMSA2NS4zNTM5IDEyMC4wMjQgNzguMDc1OEMxMjEuNzk2IDgzLjY2NDMgMTIyLjEwOCA4OS42MTI4IDEyMC45MjkgOTUuMzU1NUMxMjAuODE0IDk1LjkxNzcgMTIwLjY4NSA5Ni40NzU4IDEyMC41NDEgOTcuMDNDMTIwLjQwMSA5Ny41Nzg2IDEyMC4yNDUgOTguMTIxMSAxMjAuMDggOTguNjYxNUwxMTQuMzM4IDk1LjM2MDhMMTE4LjA5NiAxMTcuMzM4TDEzOSAxMDkuNTM5TDEzMy43NDcgMTA2LjUxOEMxMzMuOTQ5IDEwNi4wMDEgMTM0LjE1IDEwNS40ODQgMTM0LjMzNiAxMDQuOTZaIiBmaWxsPSJ3aGl0ZSIvPgo8L3N2Zz4=",
        name: "Cycles",
        owner: ic::id(),
        symbol: "XTC",
        totalSupply: StatsData::get().supply
    }
}

#[update]
fn meta_certified() -> TokenMetaData<'static> {
    meta()
}

#[query(name = "nameErc20")]
fn name_erc20() -> &'static str {
    get_metadata().name
}

#[query]
fn symbol() -> &'static str {
    get_metadata().symbol
}

#[query]
pub async fn decimals() -> u8 {
    get_metadata().decimals
}

#[query]
fn logo() -> &'static str {
    get_metadata().logo
}
