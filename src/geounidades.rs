use std::error::Error;
use std::collections::HashMap;
use shapefile::dbase::{Record,FieldValue};
use geo::algorithm::centroid::Centroid;

use crate::geodata::Datos;

#[derive(Clone,Debug)]
pub struct GeoPoligono<T: Datos> {
    pub poligono: geo::Polygon<f64>,
    pub datos: Option<T>,
    pub centroide: Option<geo::Point<f64>>,
}

impl<T: Datos> GeoPoligono<T> {
    pub fn new(geopol: &geo::Polygon<f64>, datos: Option<T>) -> Self {

        let centroide = geopol.centroid();

        GeoPoligono {
            poligono: geopol.clone(),
            datos: datos,
            centroide: centroide,
        }
    }
}

pub fn read_polygons<T: Datos>(ruta: &str, mapa: &HashMap<String,T>, cveid: &str) -> Result<Vec<GeoPoligono<T>>, Box<dyn Error>>
    where T: Clone,
{
    let pols = shapefile::read_as::<_,shapefile::Polygon, Record>(ruta)?;

    let mut salida: Vec<GeoPoligono<T>> = Vec::new();

    for (polygon, record) in pols {
        let geo_polygon: geo::MultiPolygon<f64> = polygon.into();
        for geopol in geo_polygon {

            let datos = match record.get(cveid) {
                Some(FieldValue::Character(Some(clave))) => {
                    mapa.get(clave).cloned()
                },
                _ => None,
            };

            let poltemp = GeoPoligono::new(&geopol, datos);
            salida.push(poltemp);
        }
    }

    Ok(salida)

}

#[derive(Clone,Debug)]
pub struct GeoPunto<T: Datos> {
    pub punto: geo::Point<f64>,
    pub datos: Option<T>,
}

impl<T: Datos> GeoPunto<T> {
    pub fn new(geopoint: &geo::Point<f64>, datos: Option<T>) -> Self {

        GeoPunto {
            punto: geopoint.clone(),
            datos: datos,
        }
    }
}

pub fn points_from_vec<T: Datos>(vector: Vec<T>) -> Result<Vec<GeoPunto<T>>, Box<dyn Error>>
{

    let mut salida: Vec<GeoPunto<T>> = Vec::new();

    for evento in vector {
        let coords = match evento.get_coordinates() {
            Some((lon,lat)) => (lon,lat),
            _ => break
        };

        let punto = geo::Point::new(coords.0, coords.1);

        let pointemp = GeoPunto::new(&punto, Some(evento));
        salida.push(pointemp);
    };

    Ok(salida)
}