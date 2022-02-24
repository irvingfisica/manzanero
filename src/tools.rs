#[cfg(feature = "proj")]
use std::error::Error;
use std::collections::HashMap;
#[cfg(feature = "proj")]
use proj::Proj;
#[cfg(feature = "proj")]
use geo_types::{Point,LineString,Polygon,MultiPolygon};
use crate::geounidades;
use geounidades::GeoTool;

pub fn to_deg(rads: f64) -> f64 {
    rads * std::f64::consts::FRAC_1_PI * 180.0
}

pub fn to_rad(degs: f64) -> f64 {
    degs * std::f64::consts::PI / 180.0
}

#[cfg(feature = "proj")]
pub fn point_to_geodetic(punto: Point<f64>, prjstr: &str) -> Result<Point<f64>, Box<dyn Error>> {

    let prj = Proj::new(prjstr).ok_or("Hubo un error en la definición de la proyección")?;

    let result = prj.project(punto, true)?;
    let salida = result.to_degrees();

    Ok(salida)
}

#[cfg(feature = "proj")]
pub fn point_to_projected(punto: Point<f64>, prjstr: &str) -> Result<Point<f64>, Box<dyn Error>> {

    let prj = Proj::new(prjstr).ok_or("Hubo un error en la definición de la proyección")?;

    let puntorad = punto.to_radians();
    let salida = prj.project(puntorad, false)?;

    Ok(salida)
}

#[cfg(feature = "proj")]
pub fn linestring_to_geodetic(linestring: LineString<f64>, prjstr: &str) -> Result<LineString<f64>, Box<dyn Error>> {

    let mut pointvec: Vec<Point<f64>> = Vec::new();

    for point in linestring.points_iter() {
        let spunto = match point_to_geodetic(point, prjstr) {
            Ok(punto) => punto,
            _ => continue
        };
        pointvec.push(spunto);
    }

    let salida: LineString<f64> = pointvec.into();

    Ok(salida)

}

#[cfg(feature = "proj")]
pub fn linestring_to_projected(linestring: LineString<f64>, prjstr: &str) -> Result<LineString<f64>, Box<dyn Error>> {

    let mut pointvec: Vec<Point<f64>> = Vec::new();

    for point in linestring.points_iter() {
        let spunto = match point_to_projected(point, prjstr) {
            Ok(punto) => punto,
            _ => continue
        };
        pointvec.push(spunto);
    }

    let salida: LineString<f64> = pointvec.into();

    Ok(salida)

}

#[cfg(feature = "proj")]
pub fn poligon_to_geodetic(poligono: Polygon<f64>, prjstr: &str) -> Result<Polygon<f64>, Box<dyn Error>> {

    let (exterior, interiors) = poligono.into_inner();

    let sexterior: LineString<f64> = linestring_to_geodetic(exterior, prjstr)?;
    let mut sinteriors: Vec<LineString<f64>> = Vec::new();

    for interior in interiors.into_iter() {
        let sinterior = linestring_to_geodetic(interior, prjstr)?;
        sinteriors.push(sinterior);
    };

    let salida: Polygon<f64> = Polygon::new(sexterior,sinteriors);

    Ok(salida)
}

#[cfg(feature = "proj")]
pub fn poligon_to_projected(poligono: Polygon<f64>, prjstr: &str) -> Result<Polygon<f64>, Box<dyn Error>> {

    let (exterior, interiors) = poligono.into_inner();

    let sexterior: LineString<f64> = linestring_to_projected(exterior, prjstr)?;
    let mut sinteriors: Vec<LineString<f64>> = Vec::new();

    for interior in interiors.into_iter() {
        let sinterior = linestring_to_projected(interior, prjstr)?;
        sinteriors.push(sinterior);
    };

    let salida: Polygon<f64> = Polygon::new(sexterior,sinteriors);

    Ok(salida)
}

#[cfg(feature = "proj")]
pub fn multipoligon_to_geodetic(poligonos: MultiPolygon<f64>, prjstr: &str) -> Result<MultiPolygon<f64>, Box<dyn Error>> {

    let mut salvec: Vec<Polygon<f64>> = Vec::new();

    for poligon in poligonos {
        let salpol = poligon_to_geodetic(poligon, prjstr)?;

        salvec.push(salpol);
    }

    let salida: MultiPolygon<f64> = salvec.into();

    Ok(salida)
}

#[cfg(feature = "proj")]
pub fn multipoligon_to_projected(poligonos: MultiPolygon<f64>, prjstr: &str) -> Result<MultiPolygon<f64>, Box<dyn Error>> {

    let mut salvec: Vec<Polygon<f64>> = Vec::new();

    for poligon in poligonos {
        let salpol = poligon_to_projected(poligon, prjstr)?;

        salvec.push(salpol);
    }

    let salida: MultiPolygon<f64> = salvec.into();

    Ok(salida)
}

pub fn xrange_vec<T: GeoTool>(vector: &Vec<T>) -> Option<(f64,f64)> {

    let vecexts: Vec<(f64,f64)> = vector.iter().filter_map(|ele| ele.xrange()).collect();

    let min = vecexts.iter().map(|ext| ext.0).fold(f64::INFINITY, f64::min);
    let max = vecexts.iter().map(|ext| ext.1).fold(f64::NEG_INFINITY, f64::max);

    Some((min,max))

}

pub fn yrange_vec<T: GeoTool>(vector: &Vec<T>) -> Option<(f64,f64)> {

    let vecexts: Vec<(f64,f64)> = vector.iter().filter_map(|ele| ele.yrange()).collect();

    let min = vecexts.iter().map(|ext| ext.0).fold(f64::INFINITY, f64::min);
    let max = vecexts.iter().map(|ext| ext.1).fold(f64::NEG_INFINITY, f64::max);

    Some((min,max))

}

pub fn xrange_map<S,T: GeoTool>(mapa: &HashMap<S,T>) -> Option<(f64,f64)> {

    let vecexts: Vec<(f64,f64)> = mapa.values().filter_map(|ele| ele.xrange()).collect();

    let min = vecexts.iter().map(|ext| ext.0).fold(f64::INFINITY, f64::min);
    let max = vecexts.iter().map(|ext| ext.1).fold(f64::NEG_INFINITY, f64::max);

    Some((min,max))

}

pub fn yrange_map<S,T: GeoTool>(mapa: &HashMap<S,T>) -> Option<(f64,f64)> {

    let vecexts: Vec<(f64,f64)> = mapa.values().filter_map(|ele| ele.yrange()).collect();

    let min = vecexts.iter().map(|ext| ext.0).fold(f64::INFINITY, f64::min);
    let max = vecexts.iter().map(|ext| ext.1).fold(f64::NEG_INFINITY, f64::max);

    Some((min,max))

}