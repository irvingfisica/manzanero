use std::error::Error;
use proj::Proj;
use geo_types::{Point,LineString,Polygon,MultiPolygon};

pub fn to_deg(rads: f64) -> f64 {
    rads * std::f64::consts::FRAC_1_PI * 180.0
}

pub fn to_rad(degs: f64) -> f64 {
    degs * std::f64::consts::PI / 180.0
}

pub fn point_to_geodetic(punto: Point<f64>, prjstr: &str) -> Result<Point<f64>, Box<dyn Error>> {

    let prj = Proj::new(prjstr).ok_or("Hubo un error en la definición de la proyección")?;

    let result = prj.project(punto, true)?;
    let salida = result.to_degrees();

    Ok(salida)
}

pub fn point_to_projected(punto: Point<f64>, prjstr: &str) -> Result<Point<f64>, Box<dyn Error>> {

    let prj = Proj::new(prjstr).ok_or("Hubo un error en la definición de la proyección")?;

    let puntorad = punto.to_radians();
    let salida = prj.project(puntorad, false)?;

    Ok(salida)
}

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

pub fn multipoligon_to_geodetic(poligonos: MultiPolygon<f64>, prjstr: &str) -> Result<MultiPolygon<f64>, Box<dyn Error>> {

    let mut salvec: Vec<Polygon<f64>> = Vec::new();

    for poligon in poligonos {
        let salpol = poligon_to_geodetic(poligon, prjstr)?;

        salvec.push(salpol);
    }

    let salida: MultiPolygon<f64> = salvec.into();

    Ok(salida)
}

pub fn multipoligon_to_projected(poligonos: MultiPolygon<f64>, prjstr: &str) -> Result<MultiPolygon<f64>, Box<dyn Error>> {

    let mut salvec: Vec<Polygon<f64>> = Vec::new();

    for poligon in poligonos {
        let salpol = poligon_to_projected(poligon, prjstr)?;

        salvec.push(salpol);
    }

    let salida: MultiPolygon<f64> = salvec.into();

    Ok(salida)
}