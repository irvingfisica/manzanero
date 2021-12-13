use std::error::Error;
use std::collections::HashMap;
use shapefile::dbase::{Record,FieldValue};
use geo::algorithm::centroid::Centroid;
use kdtree::KdTree;

use crate::geodata::Datos;

pub trait GeoTool {
    fn agregar_rama(&self, arbol: &mut KdTree<f64, String, [f64;2]>) -> Result<(), Box<dyn Error>>;
}

#[derive(Clone,Debug)]
pub struct GeoPoligono<T: Datos> {
    pub poligono: geo::Polygon<f64>,
    pub datos: Option<T>,
    pub centroide: Option<geo::Point<f64>>,
    pub cve: Option<String>,
}

impl<T: Datos> GeoPoligono<T> {
    pub fn new(geopol: &geo::Polygon<f64>, datos: Option<T>, cve: Option<String>) -> Self {

        let centroide = geopol.centroid();

        GeoPoligono {
            poligono: geopol.clone(),
            datos: datos,
            centroide: centroide,
            cve: cve,
        }
    }
}

impl<T: Datos> GeoTool for GeoPoligono<T> {
    fn agregar_rama(&self, arbol: &mut KdTree<f64, String, [f64;2]>) -> Result<(), Box<dyn Error>> {

        match (self.centroide,self.cve.as_ref()) {
            (Some(punto),Some(cve)) => {
                let coord = [punto.x(),punto.y()];
                arbol.add(coord,cve.clone())?;
            },
            _ => {}
        };

        Ok(())
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

            let mut cve = None; 

            let datos = match record.get(cveid) {
                Some(FieldValue::Character(Some(clave))) => {
                    cve = Some(clave.clone());
                    mapa.get(clave).cloned()
                },
                _ => None,
            };

            let poltemp = GeoPoligono::new(&geopol, datos, cve);
            salida.push(poltemp);
        }
    }

    Ok(salida)

}

#[derive(Clone,Debug)]
pub struct GeoPunto<T: Datos> {
    pub punto: Option<geo::Point<f64>>,
    pub datos: Option<T>,
    pub cve: Option<String>,
}

impl<T: Datos> GeoPunto<T> {
    pub fn new(geopoint: Option<geo::Point<f64>>, datos: Option<T>, cve: Option<String>) -> Self {

        GeoPunto {
            punto: geopoint.clone(),
            datos: datos,
            cve: cve,
        }
    }
}

impl<T: Datos> GeoTool for GeoPunto<T> {
    fn agregar_rama(&self, arbol: &mut KdTree<f64, String, [f64;2]>) -> Result<(), Box<dyn Error>> {

        match (self.punto,self.cve.as_ref()) {
            (Some(punto),Some(cve)) => {
                let coord = [punto.x(),punto.y()];
                arbol.add(coord,cve.clone())?;
            },
            _ => {}
        };

        Ok(())
    }
}

pub fn points_from_vec<T: Datos>(vector: Vec<T>) -> Result<Vec<GeoPunto<T>>, Box<dyn Error>>
{

    let mut salida: Vec<GeoPunto<T>> = Vec::new();

    let mut contador: usize = 0;

    for evento in vector {
        let coords = match evento.get_coordinates() {
            Some((lon,lat)) => {
                contador = contador + 1;
                (lon,lat)
            },
            _ => break
        };

        let punto = Some(geo::Point::new(coords.0, coords.1));
        let cve = contador.to_string();

        let pointemp = GeoPunto::new(punto, Some(evento), Some(cve));
        salida.push(pointemp);
    };

    Ok(salida)
}