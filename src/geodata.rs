use serde::{Deserialize};
use std::collections::HashMap;
use std::error::Error;
use crate::geounidades::GeoMultiPoligono;
use shapefile::dbase::{Record,FieldValue};

pub trait Datos {
    fn get_coordinates(&self) -> Option<(f64,f64)>;
    fn get_cve(&self) -> Option<String>;
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Clone)]
pub struct ManzanaCenso2020 {
    #[serde(rename = "ENTIDAD")]
    pub entidad: String,
    #[serde(rename = "MUN")]
    pub municipio: String,
    #[serde(rename = "LOC")]
    pub localidad: String,
    #[serde(rename = "AGEB")]
    pub ageb: String,
    #[serde(rename = "MZA")]
    pub manzana: String,
    pub cve: Option<String>,
    pub POBTOT: Option<i32>,
    pub POBFEM: Option<i32>,
    pub POBMAS: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P_0A2: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P_0A2_F: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P_0A2_M: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P_3YMAS: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P_3YMAS_F: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P_3YMAS_M: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P_5YMAS: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P_5YMAS_F: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P_5YMAS_M: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P_12YMAS: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P_12YMAS_F: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P_12YMAS_M: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P_15YMAS: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P_15YMAS_F: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P_15YMAS_M: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P_18YMAS: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P_18YMAS_F: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P_18YMAS_M: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P_3A5: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P_3A5_F: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P_3A5_M: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P_6A11: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P_6A11_F: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P_6A11_M: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P_8A14: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P_8A14_F: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P_8A14_M: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P_12A14: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P_12A14_F: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P_12A14_M: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P_15A17: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P_15A17_F: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P_15A17_M: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P_18A24: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P_18A24_F: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P_18A24_M: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P_15A49_F: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P_60YMAS: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P_60YMAS_F: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P_60YMAS_M: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub REL_H_M: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub POB0_14: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub POB15_64: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub POB65_MAS: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PROM_HNV: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PNACENT: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PNACENT_F: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PNACENT_M: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PNACOE: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PNACOE_F: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PNACOE_M: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PRES2015: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PRES2015_F: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PRES2015_M: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PRESOE15: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PRESOE15_F: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PRESOE15_M: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P3YM_HLI: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P3YM_HLI_F: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P3YM_HLI_M: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P3HLINHE: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P3HLINHE_F: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P3HLINHE_M: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P3HLI_HE: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P3HLI_HE_F: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P3HLI_HE_M: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P5_HLI: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P5_HLI_NHE: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P5_HLI_HE: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PHOG_IND: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub POB_AFRO: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub POB_AFRO_F: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub POB_AFRO_M: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PCON_DISC: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PCDISC_MOT: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PCDISC_VIS: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PCDISC_LENG: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PCDISC_AUD: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PCDISC_MOT2: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PCDISC_MEN: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PCON_LIMI: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PCLIM_CSB: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PCLIM_VIS: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PCLIM_HACO: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PCLIM_OAUD: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PCLIM_MOT2: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PCLIM_RE_CO: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PCLIM_PMEN: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PSIND_LIM: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P3A5_NOA: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P3A5_NOA_F: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P3A5_NOA_M: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P6A11_NOA: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P6A11_NOAF: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P6A11_NOAM: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P12A14NOA: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P12A14NOAF: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P12A14NOAM: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P15A17A: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P15A17A_F: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P15A17A_M: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P18A24A: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P18A24A_F: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P18A24A_M: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P8A14AN: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P8A14AN_F: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P8A14AN_M: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P15YM_AN: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P15YM_AN_F: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P15YM_AN_M: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P15YM_SE: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P15YM_SE_F: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P15YM_SE_M: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P15PRI_IN: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P15PRI_INF: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P15PRI_INM: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P15PRI_CO: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P15PRI_COF: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P15PRI_COM: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P15SEC_IN: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P15SEC_INF: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P15SEC_INM: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P15SEC_CO: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P15SEC_COF: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P15SEC_COM: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P18YM_PB: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P18YM_PB_F: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P18YM_PB_M: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub GRAPROES: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub GRAPROES_F: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub GRAPROES_M: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PEA: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PEA_F: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PEA_M: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PE_INAC: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PE_INAC_F: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PE_INAC_M: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub POCUPADA: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub POCUPADA_F: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub POCUPADA_M: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PDESOCUP: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PDESOCUP_F: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PDESOCUP_M: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PSINDER: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PDER_SS: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PDER_IMSS: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PDER_ISTE: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PDER_ISTEE: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PAFIL_PDOM: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PDER_SEGP: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PDER_IMSSB: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PAFIL_IPRIV: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PAFIL_OTRAI: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P12YM_SOLT: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P12YM_CASA: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub P12YM_SEPA: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PCATOLICA: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PRO_CRIEVA: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub POTRAS_REL: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PSIN_RELIG: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub TOTHOG: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub HOGJEF_F: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub HOGJEF_M: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub POBHOG: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PHOGJEF_F: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PHOGJEF_M: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub VIVTOT: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub TVIVHAB: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub TVIVPAR: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub VIVPAR_HAB: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub TVIVPARHAB: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub VIVPAR_DES: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub VIVPAR_UT: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub OCUPVIVPAR: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PROM_OCUP: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub PRO_OCUP_C: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub VPH_PISODT: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub VPH_PISOTI: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub VPH_1DOR: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub VPH_2YMASD: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub VPH_1CUART: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub VPH_2CUART: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub VPH_3YMASC: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub VPH_C_ELEC: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub VPH_S_ELEC: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub VPH_AGUADV: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub VPH_AEASP: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub VPH_AGUAFV: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub VPH_TINACO: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub VPH_CISTER: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub VPH_EXCSA: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub VPH_LETR: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub VPH_DRENAJ: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub VPH_NODREN: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub VPH_C_SERV: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub VPH_NDEAED: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub VPH_DSADMA: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub VPH_NDACMM: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub VPH_SNBIEN: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub VPH_REFRI: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub VPH_LAVAD: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub VPH_HMICRO: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub VPH_AUTOM: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub VPH_MOTO: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub VPH_BICI: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub VPH_RADIO: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub VPH_TV: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub VPH_PC: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub VPH_TELEF: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub VPH_CEL: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub VPH_INTER: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub VPH_STVP: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub VPH_SPMVPI: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub VPH_CVJ: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub VPH_SINRTV: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub VPH_SINLTC: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub VPH_SINCINT: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub VPH_SINTIC: Option<i32>,
}

impl Datos for ManzanaCenso2020 {
    fn get_coordinates(&self) -> Option<(f64, f64)> {
        None
    }

    fn get_cve(&self) -> Option<String> {
        self.cve.clone()
    }
}

pub fn read_mzacenso2020_csv(ruta: &str, mapa: &mut HashMap<String,ManzanaCenso2020>) -> Result<(), Box<dyn Error>> 
{

    let mut rdr = csv::ReaderBuilder::new()
                    .from_path(ruta)?;

    let headers = rdr.headers()?.clone();

    for result in rdr.records() {
        match result {
            Ok(record) => {
                let mut new_record = match record.deserialize::<ManzanaCenso2020>(Some(&headers)) {
                    Ok(new_record) => {
                        new_record
                    },
                    Err(_) => {
                        continue
                    },
                };

                let mut cve = new_record.entidad.clone();
                cve.push_str(&new_record.municipio);
                cve.push_str(&new_record.localidad);
                cve.push_str(&new_record.ageb);
                cve.push_str(&new_record.manzana);
                new_record.cve = Some(cve.clone());

                mapa.insert(cve, new_record);
            },
            Err(_) => continue
        }
    }

    Ok(())
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Clone)]
pub struct CarpetaInvestigacionCDMX {
    pub ao_hechos: i32,
    pub mes_hechos: String,
    pub fecha_hechos: String,
    pub ao_inicio: i32,
    pub mes_inicio: String,
    pub fecha_inicio: String,
    pub delito: String,
    pub fiscalia: String,
    pub agencia: String,
    pub unidad_investigacion: String,
    pub categoria_delito: String,
    pub calle_hechos: String,
    pub calle_hechos2: String,
    pub colonia_hechos: String,
    pub alcaldia_hechos: String,
    pub longitud: f64,
    pub latitud: f64,
}

impl Datos for CarpetaInvestigacionCDMX {
    fn get_coordinates(&self) -> Option<(f64, f64)> {
        Some((self.longitud,self.latitud))
    }

    fn get_cve(&self) -> Option<String> {
        None
    }
}

pub fn read_carpetascdmx_csv(ruta: &str, vector: &mut Vec<CarpetaInvestigacionCDMX>) -> Result<(), Box<dyn Error>> 
{

    let mut rdr = csv::ReaderBuilder::new()
                    .from_path(ruta)?;

    let headers = rdr.headers()?.clone();

    for result in rdr.records() {
        match result {
            Ok(record) => {
                let new_record = match record.deserialize::<CarpetaInvestigacionCDMX>(Some(&headers)) {
                    Ok(new_record) => {
                        new_record
                    },
                    Err(_) => {
                        continue
                    },
                };

                vector.push(new_record);
            },
            Err(_) => continue
        }
    }

    Ok(())
}

#[derive(Debug, Clone)]
pub struct SeccionBm {
    pub cvesec: String,
    pub ventil: String,
    pub ventil_nacional: String,
    pub lista_nominal: Option<i32>,
}

impl SeccionBm {
    pub fn new(cve: &str, ventil: &str, ventil_nacional: &str, lista_nominal: Option<i32>) -> Self {
        SeccionBm {
            cvesec: cve.to_string(),
            ventil: ventil.to_string(),
            ventil_nacional: ventil_nacional.to_string(),
            lista_nominal: lista_nominal.clone()
        }
    }
}

impl Datos for SeccionBm {
    fn get_coordinates(&self) -> Option<(f64, f64)> {
        None
    }

    fn get_cve(&self) -> Option<String> {
        Some(self.cvesec.to_string())
    }
}

pub fn read_secciones_bm(ruta: &str) -> Result<HashMap<String,GeoMultiPoligono<SeccionBm>>, Box<dyn Error>>
{
    let pols = shapefile::read_as::<_,shapefile::Polygon, Record>(ruta)?;

    let mut salida: HashMap<String,GeoMultiPoligono<SeccionBm>> = HashMap::new();

    for (polygon, record) in pols {

        let cvesec = match record.get("CVE") {
            Some(FieldValue::Character(Some(clave))) => clave.to_string(),
            _ => continue,
        };

        let ventil = match record.get("Ventil") {
            Some(FieldValue::Character(Some(vent))) => vent.to_string(),
            _ => "-".to_string(),
        };

        let ventil_nacional = match record.get("Ventil_Nac") {
            Some(FieldValue::Character(Some(vent))) => vent.to_string(),
            _ => "-".to_string(),
        };

        let lista_nominal = match record.get("LNominal") {
            Some(FieldValue::Character(Some(lista))) => lista.parse::<i32>().ok(),
            _ => None,
        };

        let seccion = SeccionBm::new(&cvesec,&ventil,&ventil_nacional,lista_nominal);

        let geo_polygon: geo::MultiPolygon<f64> = polygon.into();

        let cve = seccion.get_cve().unwrap();

        let geosec = GeoMultiPoligono::new(&geo_polygon, Some(seccion), &cve);

        salida.insert(cve.to_string(),geosec);
    };

    Ok(salida)
}

