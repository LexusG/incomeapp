slint::include_modules!();

const TAXPERC:       f64 =0.30;
const OWNERPERC:     f64 =0.55;
const PROFITPERC:    f64 =0.05;
const OPERATIONXPERC: f64 =0.10;


fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

        let ui_handle = ui.as_weak();
        ui.on_divide_income(move |string| {
            let ui = ui_handle.unwrap();
            let num: f64 = string.trim().parse().unwrap();
            let tax: f64 = num * TAXPERC;
            let owner: f64 = num * OWNERPERC;
            let profit: f64 = num * PROFITPERC;
            let operation_x: f64 = num * OPERATIONXPERC;
            let result: String = format!("Taxes: {:.2}\nOwner: {:.2}\nProfit: {:.2}\nOperationX:{:.2}", tax, owner, profit, operation_x);

            ui.set_results(result.into());
        });
    

    ui.run()
}
