use std::error::Error;
use std::collections::HashMap;
use shapefile::dbase::{Record,FieldValue};
use geo::algorithm::centroid::Centroid;
use geo::algorithm::bounding_rect::BoundingRect;
use geo::extremes::Extremes;
use geo_types::Rect;
use kdtree::KdTree;
use crate::tools;
use tools::{multipoligon_to_geodetic,multipoligon_to_projected};
use tools::{poligon_to_geodetic,poligon_to_projected};
use tools::{point_to_geodetic,point_to_projected};

use crate::geodata::Datos;

pub trait Projector {
    fn to_geodetic(&mut self, prjstr: &str) -> Result<(), Box<dyn Error>>;
    fn to_projected(&mut self, prjstr: &str) -> Result<(), Box<dyn Error>>;
}

pub trait GeoTool {
    fn agregar_rama(&self, arbol: &mut KdTree<f64, String, [f64;2]>) -> Result<(), Box<dyn Error>>;
    fn xrange(&self) -> Option<(f64,f64)>;
    fn yrange(&self) -> Option<(f64,f64)>;
    fn get_bounding(&self) -> Option<Rect<f64>>;
}

#[derive(Clone,Debug)]
pub struct GeoMultiPoligono<T: Datos> {
    pub poligono: geo::MultiPolygon<f64>,
    pub datos: Option<T>,
    pub centroide: Option<geo::Point<f64>>,
    pub cve: String,
}

impl<T: Datos> GeoMultiPoligono<T> {
    pub fn new(geopol: &geo::MultiPolygon<f64>, datos: Option<T>, cve: &str) -> Self {

        let centroide = geopol.centroid();

        GeoMultiPoligono {
            poligono: geopol.clone(),
            datos: datos,
            centroide: centroide,
            cve: cve.to_string(),
        }
    }
}

impl<T: Datos> GeoTool for GeoMultiPoligono<T> {
    fn agregar_rama(&self, arbol: &mut KdTree<f64, String, [f64;2]>) -> Result<(), Box<dyn Error>> {

        match self.centroide {
            Some(punto) => {
                let coord = [punto.x(),punto.y()];
                arbol.add(coord,self.cve.clone())?;
            },
            _ => {}
        };

        Ok(())
    }

    fn xrange(&self) -> Option<(f64,f64)> {
        match self.poligono.extremes() {
            Some(extremes) => {
                Some((extremes.x_min.coord.x,extremes.x_max.coord.x))
            },
            None => None
        }
    }

    fn yrange(&self) -> Option<(f64,f64)> {
        match self.poligono.extremes() {
            Some(extremes) => {
                Some((extremes.y_min.coord.y,extremes.y_max.coord.y))
            },
            None => None
        }
    }

    fn get_bounding(&self) -> Option<Rect<f64>> {
        self.poligono.bounding_rect()
    }
}

impl<T: Datos> Projector for GeoMultiPoligono<T> {
    fn to_geodetic(&mut self, prjstr: &str) -> Result<(), Box<dyn Error>> {

        let smpol = multipoligon_to_geodetic(self.poligono.clone(), prjstr)?;
        self.poligono = smpol;

        Ok(())
    }

    fn to_projected(&mut self, prjstr: &str) -> Result<(), Box<dyn Error>> {

        let smpol = multipoligon_to_projected(self.poligono.clone(), prjstr)?;
        self.poligono = smpol;

        Ok(())
    }
}

#[derive(Clone,Debug)]
pub struct GeoPoligono<T: Datos> {
    pub poligono: geo::Polygon<f64>,
    pub datos: Option<T>,
    pub centroide: Option<geo::Point<f64>>,
    pub cve: String,
}

impl<T: Datos> GeoPoligono<T> {
    pub fn new(geopol: &geo::Polygon<f64>, datos: Option<T>, cve: &str) -> Self {

        let centroide = geopol.centroid();

        GeoPoligono {
            poligono: geopol.clone(),
            datos: datos,
            centroide: centroide,
            cve: cve.to_string(),
        }
    }
}

impl<T: Datos> GeoTool for GeoPoligono<T> {
    fn agregar_rama(&self, arbol: &mut KdTree<f64, String, [f64;2]>) -> Result<(), Box<dyn Error>> {

        match self.centroide {
            Some(punto) => {
                let coord = [punto.x(),punto.y()];
                arbol.add(coord,self.cve.clone())?;
            },
            _ => {}
        };

        Ok(())
    }

    fn xrange(&self) -> Option<(f64,f64)> {
        match self.poligono.extremes() {
            Some(extremes) => {
                Some((extremes.x_min.coord.x,extremes.x_max.coord.x))
            },
            None => None
        }
    }

    fn yrange(&self) -> Option<(f64,f64)> {
        match self.poligono.extremes() {
            Some(extremes) => {
                Some((extremes.y_min.coord.y,extremes.y_max.coord.y))
            },
            None => None
        }
    }

    fn get_bounding(&self) -> Option<Rect<f64>> {
        self.poligono.bounding_rect()
    }
}

impl<T: Datos> Projector for GeoPoligono<T> {
    fn to_geodetic(&mut self, prjstr: &str) -> Result<(), Box<dyn Error>> {

        let smpol = poligon_to_geodetic(self.poligono.clone(), prjstr)?;
        self.poligono = smpol;

        Ok(())
    }

    fn to_projected(&mut self, prjstr: &str) -> Result<(), Box<dyn Error>> {

        let smpol = poligon_to_projected(self.poligono.clone(), prjstr)?;
        self.poligono = smpol;

        Ok(())
    }
}

pub fn read_polygons<T: Datos>(ruta: &str, mapa: &HashMap<String,T>, cveid: &str) -> Result<HashMap<String,GeoPoligono<T>>, Box<dyn Error>>
    where T: Clone,
{
    let pols = shapefile::read_as::<_,shapefile::Polygon, Record>(ruta)?;

    let mut salida: HashMap<String,GeoPoligono<T>> = HashMap::new();

    let mut contadora: usize = 0;
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

            match cve {
                Some(clave) => {
                    let poltemp = GeoPoligono::new(&geopol, datos, &clave);
                    salida.insert(clave,poltemp);
                },
                None => {
                    contadora = contadora + 1;
                    let clave = contadora.to_string();
                    let poltemp = GeoPoligono::new(&geopol, datos, &clave);
                    salida.insert(clave,poltemp);
                }
            }
        }
    }

    Ok(salida)

}

#[derive(Clone,Debug)]
pub struct GeoPunto<T: Datos> {
    pub punto: geo::Point<f64>,
    pub datos: Option<T>,
    pub cve: String,
}

impl<T: Datos> GeoPunto<T> {
    pub fn new(geopoint: geo::Point<f64>, datos: Option<T>, cve: &str) -> Self {

        GeoPunto {
            punto: geopoint.clone(),
            datos: datos,
            cve: cve.to_string(),
        }
    }
}

impl<T: Datos> GeoTool for GeoPunto<T> {
    fn agregar_rama(&self, arbol: &mut KdTree<f64, String, [f64;2]>) -> Result<(), Box<dyn Error>> {

        let coord = [self.punto.x(),self.punto.y()];
        arbol.add(coord,self.cve.clone())?;

        Ok(())
    }

    fn xrange(&self) -> Option<(f64,f64)> {
        Some((self.punto.x(),self.punto.y()))
    }

    fn yrange(&self) -> Option<(f64,f64)> {
        Some((self.punto.x(),self.punto.y()))
    }

    fn get_bounding(&self) -> Option<Rect<f64>> {
        Some(self.punto.bounding_rect())
    }
}

impl<T: Datos> Projector for GeoPunto<T> {
    fn to_geodetic(&mut self, prjstr: &str) -> Result<(), Box<dyn Error>> {

        let smpol = point_to_geodetic(self.punto.clone(), prjstr)?;
        self.punto = smpol;

        Ok(())
    }

    fn to_projected(&mut self, prjstr: &str) -> Result<(), Box<dyn Error>> {

        let smpol = point_to_projected(self.punto.clone(), prjstr)?;
        self.punto = smpol;

        Ok(())
    }
}

pub fn points_from_vec<T: Datos>(vector: Vec<T>) -> Result<HashMap<String,GeoPunto<T>>, Box<dyn Error>>
{

    let mut salida: HashMap<String,GeoPunto<T>> = HashMap::new();

    let mut contador: usize = 0;

    for evento in vector {
        let coords = match evento.get_coordinates() {
            Some((lon,lat)) => {
                (lon,lat)
            },
            _ => break
        };

        let punto = geo::Point::new(coords.0, coords.1);
        let cve = match evento.get_cve() {
            None => {
                contador = contador + 1;
                contador.to_string()
            }
            Some(clave) => {
                clave
            }
        };

        let pointemp = GeoPunto::new(punto, Some(evento), &cve);
        salida.insert(cve,pointemp);
    };

    Ok(salida)
}