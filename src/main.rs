use std::collections::HashMap;
use manzanero::geodata;
use manzanero::geounidades;
use geodata::{ManzanaCenso2020,CarpetaInvestigacionCDMX};
use geounidades::{GeoPoligono,GeoPunto};
use geounidades::Projector;

fn main() {
    let mut mapa: HashMap<String,ManzanaCenso2020> = HashMap::new();

    geodata::read_mzacenso2020_csv("./datos/RESAGEBURB_09CSV20.csv", &mut mapa).unwrap();
    let manzanas: HashMap<String, GeoPoligono<ManzanaCenso2020>> = geounidades::read_polygons("./datos/09m.shp", &mapa, "CVEGEO").unwrap();

    let mut carpetas: Vec<CarpetaInvestigacionCDMX> = Vec::new();

    geodata::read_carpetascdmx_csv("./datos/carpetas_completa_octubre_2021.csv", &mut carpetas).unwrap();
    let _eventos: HashMap<String, GeoPunto<CarpetaInvestigacionCDMX>> = geounidades::points_from_vec(carpetas).unwrap();

    let mut mantest = manzanas.get("0900200010010001").unwrap().clone();
    println!("Projectadas: {:?}\n",mantest.poligono);

    let prjstr = "+proj=lcc +lat_0=12 +lon_0=-102 +lat_1=17.5 +lat_2=29.5 +x_0=2500000 +y_0=0 +ellps=GRS80 +units=m +no_defs";

    mantest.to_geodetic(prjstr).unwrap();
    println!("Geodeticas: {:?}",mantest.poligono);

}

// use proj::Proj;

// let ft_to_m = Proj::new("+proj=pipeline +step +inv +proj=lcc +lat_0=12 +lon_0=-102 +lat_1=17.5 +lat_2=29.5 +x_0=2500000 +y_0=0 +ellps=GRS80 +step +proj=unitconvert +xy_in=rad +xy_out=deg").unwrap();

// let result = ft_to_m.project((-99.23031311644314f64, 19.359209026456842f64),true).unwrap();

// println!("{:?}",result);