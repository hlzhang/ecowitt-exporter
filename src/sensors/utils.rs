pub fn temp_f_to_c(f: f64) -> f64 {
    round_to_x_places((f - 32.0) / 1.8, 1)
}

pub fn press_inhg_to_pa(inhg: f64) -> f64 {
    (inhg / 0.00029530).round()
}

pub fn speed_mph_to_kmph(mph: f64) -> f64 {
    round_to_x_places(mph * 1.60934, 2)
}

pub fn length_in_to_mm(inch: f64) -> f64 {
    round_to_x_places(inch * 25.4, 1)
}

// Round converted floats to a specific decimal place
// this avoids giving any false idea about metric accuracy
fn round_to_x_places(value: f64, power: u32) -> f64 {
    let base: i64 = 10;
    let multiplier: f64 = base.pow(power) as f64;
    f64::round(value * multiplier) / multiplier
}

// Calculation Of Vapour Pressure Deficit
// To calculate VPD for a given temperature (degC) and relative humidity (RH%)
// (1) Ascertain the saturated vapour pressure (SVP) for a given temperature (see list below)
// Temperature (degC) - SVP (Pa)
// 0 - 611
// 1 - 657
// 2 - 706
// 3 - 758
// 4 - 813
// 5 - 872
// 6 - 935
// 7 - 1002
// 8 - 1073
// 9 - 1148
// 10 - 1228
// 11 - 1312
// 12 - 1402
// 13 - 1497
// 14 - 1598
// 15 - 1705
// 16 - 1818
// 17 - 1937
// 18 - 2064
// 19 - 2197
// 20 - 2338
// 21 - 2486
// 22 - 2643
// 23 - 2809
// 24 - 2983
// 25 - 3167
// 26 - 3361
// 27 - 3565
// 28 - 3779
// 29 - 4005
// 30 - 4242
// 31 - 4492
// 32 - 4754
// 33 - 5029
// 34 - 5318
// 35 - 5621
// 36 - 5940
// 37 - 6273
// 38 - 6623
// 39 - 6990
// 40 - 7374
// 41 - 7776
// Source: http://physics.holsoft.nl/physics/ocmain.htm
// See: Murray FW (1967) On the computation of saturation vapor pressure. J. Appl. Meteorol. 6: 203-204.
// Monteith JL, Unsworth MH (1990) Principles of environmental physics. Arnold.
// SVP (Pascals) = 610.7*10^7.5T/(237.3+T)
//
// (2) As VPD is the saturated vapour pressure minus the actual vapour pressure (SVP - VPactual), and VPactual = (RH*SVP)/100
// we may apply the formula
// VPD = ((100 - RH)/100)*SVP
// (alternatively
// VPD = (1 - (RH/100))*SVP
// )
// where RH is relative humidity and SVP is saturated vapour pressure
//
// For example take 80% RH at 25 C. From the list above, SVP = 3167 Pa
// 100-RH = 20
// 20/100 = 0.2
// 0.2 * 3167 = 633.4 pascals (Pa)
// Note: for convenience VPD may be given in kilopascals (kPa). For this example 0.6334 kPa.
//
// see: http://cronklab.wikidot.com/calculation-of-vapour-pressure-deficit
//
// 饱和水汽压差（Vapor Pressure Deficit，VPD）表示的是实际空气距离水汽饱和状态的程度，即空气的干燥程度。
// 已有相关研究表明，VPD在决定植物生理功能中起着关键作用（Cunningham, 2004; 陈彪等, 2015; 赵卉忱等, 2020）。
// 首先，VPD对植物叶片的气孔导度有直接影响，VPD增大能够促进叶片表面的气孔张开，
// 有利于植被更好地吸收水分以进行光合作用和正常生理活动，但VPD超过一定阈值时又将导致植物降低气孔开度以阻止过多的水分流失，
// 从而抑制植物的生长（闫敏等, 2016）；
// 其次，VPD还与植物液流、水分利用效率等有显著的相关关系（李静, 2014; 秦奔奔和景元书, 2017）。
// 研究指出，地球的理想VPD范围在0.45～1.24 kPa，
// 最佳为0.85 kPa；植物能够较好生长的VPD范围是0.8～0.95 kPa（李静, 2014），
// 而实际中的环境因子经常无法满足植物进行最佳生长的条件。
// 因此，植物对环境中VPD的响应受到越来越多研究者的关注。
// http://www.iapjournals.ac.cn/qhhj/cn/article/doi/10.3878/j.issn.1006-9585.2021.20086
// https://link.springer.com/article/10.1007/s00468-004-0318-y
//
// SVP in Pa
pub fn svp(tempc: f64) -> f64 {
    // float svp = 610.7 * pow(10, (7.5 * t / (237.3 + t)));
    let svp: f64 = 610.7_f64 * 10.0_f64.powf(7.5_f64 * tempc/ (237.3_f64 + tempc));
    return round_to_x_places(svp, 2);
}

// VPD in kPa
pub fn vpd(tempc: f64, humidity: i64) -> f64 {
    // float vpd = (((100 - h) / 100) * svp) / 1000;
    let svp: f64 = svp(tempc);
    let vpd: f64 = (((100.0_f64 - humidity as f64) / 100.0_f64) * svp) / 1000.0_f64;
    return round_to_x_places(vpd, 2);
}

// cargo test -- --nocapture
#[cfg(test)]
mod tests {
    use crate::sensors::utils::*;

    #[test]
    fn svp_test() {
        let svp_0 = svp(0.0);
        println!("SVP for 0 C is {:?}", svp_0);
        assert_eq!(svp_0, 610.7_f64);

        let svp_25 = svp(25.0);
        println!("SVP for 25 C is {:?}", svp_25);
        assert_eq!(svp_25, 3167.07_f64);

        let svp_41 = svp(41.0);
        println!("SVP for 41 C is {:?}", svp_41);
        assert_eq!(svp_41, 7775.9_f64);
    }

    #[test]
    fn vpd_test() {
        let vpd_0_90 = vpd(0.0, 90);
        println!("VPD for 0 C and 90% RH  is {:?}", vpd_0_90);
        assert_eq!(vpd_0_90, 0.06_f64);

        let vpd_10_90 = vpd(10.0, 90);
        println!("VPD for 10 C and 90% RH  is {:?}", vpd_10_90);
        assert_eq!(vpd_10_90, 0.12_f64);

        let vpd_10_60 = vpd(10.0, 60);
        println!("VPD for 10 C and 60% RH  is {:?}", vpd_10_60);
        assert_eq!(vpd_10_60, 0.49_f64);

        let vpd_25_60 = vpd(25.0, 60);
        println!("VPD for 25 C and 60% RH  is {:?}", vpd_25_60);
        assert_eq!(vpd_25_60, 1.27_f64);

        let vpd_25_70 = vpd(25.0, 70);
        println!("VPD for 25 C and 70% RH  is {:?}", vpd_25_70);
        assert_eq!(vpd_25_70, 0.95_f64);

        let vpd_25_80 = vpd(25.0, 80);
        println!("VPD for 25 C and 80% RH  is {:?}", vpd_25_80);
        assert_eq!(vpd_25_80, 0.63_f64);
    }
}
