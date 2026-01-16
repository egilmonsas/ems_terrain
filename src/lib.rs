mod mesh;
use crate::mesh::{triangulate_grid, Mesh, Vertex};

use reqwest::Client;
use std::io::prelude::*;

// Metadata
const NAME_AUTHOR: &str = "ems";
const NAME_APPLICATION: &str = "ems_terrain";
const VERSION_APPLICATION: &str = "0.1.0";
const NAME_PROJECT: &str = "JM - Granitten";
const NAME_SITE: &str = "Karihaugveien 22";

pub struct BBox {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}
impl BBox {
    pub fn width(&self) -> f32 {
        self.x2 - self.x1
    }
    pub fn height(&self) -> f32 {
        self.y2 - self.y1
    }
    pub fn num_pixels_x(&self, resolution: f32) -> usize {
        (self.width() / resolution).ceil() as usize
    }
    pub fn num_pixels_y(&self, resolution: f32) -> usize {
        (self.height() / resolution).ceil() as usize
    }
}

// Main function to generate IFC terrain model
pub async fn generate(
    bbox: BBox,
    resolution: f32,
    coord_sys: usize,
) -> Result<Vec<u8>, reqwest::Error> {
    let mut file = Vec::new();

    write!(file, "

/* HEADER */
ISO-10303-21;
HEADER;
FILE_DESCRIPTION(('Option [Terrengmodell: On]'),'2;1');
FILE_NAME('','2023-09-04T09:29:20',('{NAME_AUTHOR}'),(''),'DDS_IFC v3.0','{NAME_APPLICATION} build {VERSION_APPLICATION}','DDS');
FILE_SCHEMA(('IFC4'));
ENDSEC;
DATA;

/* APPLICATION */
#34=IFCPERSON($,$,'',$,$,$,$,$);
#35=IFCORGANIZATION($,'',$,$,$);
#36=IFCORGANIZATION($,'Data Design System AS',$,$,$);
#6=IFCPERSONANDORGANIZATION(#34,#35,$);
#37=IFCCARTESIANPOINT((0.,0.,0.));
#7=IFCAPPLICATION(#36,'{VERSION_APPLICATION}','{NAME_APPLICATION}','{NAME_APPLICATION}');

/* PROJECT */
#1=IFCPROJECT('3rPWK2ZfD6bBH524n2KTpl',#2,'{NAME_PROJECT}',$,$,$,$,(#3),#4);
#33=IFCSITE('25Nbteh8zCUQrx_1EOgClS',$,'{NAME_SITE}',$,$,#51,$,$,.ELEMENT.,$,$,$,$,$);
#2=IFCOWNERHISTORY(#6,#7,$,.NOCHANGE.,$,$,$,1693812560);
#3=IFCGEOMETRICREPRESENTATIONCONTEXT($,'Model',3,1.E-05,#8,#9);
#5=IFCRELAGGREGATES('2kzQ1oOxb5l9$1Jt6usF8_',$,$,$,#1,(#33));
#8=IFCAXIS2PLACEMENT3D(#901,$,$);

/* MODEL */
#10=IFCGEOMETRICREPRESENTATIONSUBCONTEXT('Body','Model',*,*,*,*,#3,0.01,.MODEL_VIEW.,$);
#11=IFCMAPCONVERSION(#3,#39,0.,0.,0.,1.,0.,1.);
#51=IFCLOCALPLACEMENT($,#57);
#52=IFCRELCONTAINEDINSPATIALSTRUCTURE('2wR$JEyzD0exJeCIxHCp4q',$,$,$,(#76,#77,#78,#79,#80,#81,#82,#83,#84,#85,#86,#87,#88,#89,#90,#91,#92,#93),#33);
#54=IFCPRESENTATIONLAYERASSIGNMENT('NO_Bergmodell',$,(#53,#38),'BldBfc.3299');

/* MODEL */
#39=IFCPROJECTEDCRS('EPSG:5110','EUREF89 NTM Sone 10','EUREF89','NN2000','Gauss Kruger','EUREF89 NTM Sone 10',$);

/* UNITS */
#4=IFCUNITASSIGNMENT((#12,#13,#18,#19,#20,#26));
#12=IFCSIUNIT(*,.LENGTHUNIT.,$,.METRE.);
#13=IFCSIUNIT(*,.AREAUNIT.,$,.SQUARE_METRE.);
#18=IFCSIUNIT(*,.MASSUNIT.,.KILO.,.GRAM.);
#19=IFCSIUNIT(*,.PLANEANGLEUNIT.,$,.RADIAN.);
#20=IFCSIUNIT(*,.SOLIDANGLEUNIT.,$,.STERADIAN.);
#26=IFCSIUNIT(*,.VOLUMEUNIT.,$,.CUBIC_METRE.);

/* TERRAIN MODEL */
#38=IFCSHAPEREPRESENTATION(#10,'Body','Tessellation',(#53));
#55=IFCPRODUCTDEFINITIONSHAPE($,$,(#38));
#76=IFCGEOGRAPHICELEMENT('0AYSgff$n1XxZAfc$TvwoL',$,'Punktsky/Triangelnett - 1',$,$,#1006,#55,$,$);

/* WORLD REFERENCE POINT */
#900=IFCDIRECTION((0.,1.));
#901=IFCCARTESIANPOINT((0.,0.,0.));

/* LOCAL COORDINATE SYSTEM */
#1000=IFCCARTESIANPOINT((0.,0.,0.));
#1001=IFCDIRECTION((0.,0.,1.));
#1002=IFCDIRECTION((1.,0.,0.));
#1003=IFCAXIS2PLACEMENT3D(#1000,$,$);
#1004=IFCAXIS2PLACEMENT3D(#1000,#1001,#1002);
#1005=IFCLOCALPLACEMENT($,#1004);
#1006=IFCLOCALPLACEMENT(#1005,#1004);

/* STYLE */
#164=IFCCOLOURRGB('R:255, G:0, B:0',1.,0.,0.);
#166=IFCCOLOURRGB($,0.6823529601097107,0.09019608050584793,0.);
#165=IFCSURFACESTYLESHADING(#166,0.);
#163=IFCSURFACESTYLE('Material ID: 4.988.903',.BOTH.,(#165));
#162=IFCCURVESTYLE('2b6cb502-45af-48fa-0000-000001000228',$,$,#164,$);
#161=IFCSTYLEDITEM(#53,(#162,#163),$);

/* PROPERTIES */
#116=IFCRELASSIGNSTOGROUP('1ZvA0wPjb7KggjxvVhyCLu',$,$,$,(#76),.PRODUCT.,#138);
#117=IFCRELDEFINESBYPROPERTIES('1aJdRD0c90dBY6lurqV2b2',$,$,$,(#76),#139);
#118=IFCRELDEFINESBYPROPERTIES('2UraXKHwPEgeA643FanXJY',$,$,$,(#76),#140);
#119=IFCRELDEFINESBYPROPERTIES('3gjzP9Dj90qBpZ5WcvSd7e',$,$,$,(#76),#141);

#138=IFCGROUP('11VMw83pj9e9xctpRIFOjJ',$,'e18vk_e103_fel00_bim_mode_001_g-gf_bergmodell A03.aly',$,$);
#139=IFCPROPERTYSET('1N2c8ZgIT8qgemRZ9UedAh',$,'0_Generell',$,(#142,#143,#144,#145,#146));
#140=IFCPROPERTYSET('2c2KzWJ4H0cA24jbcVZnBr',$,'0_Element',$,(#147,#148,#149,#150,#151,#152,#153,#154));
#141=IFCPROPERTYSET('1Km3UfOgX3c9xm$_kVeGzy',$,'Attributter',$,(#155,#156,#157,#158,#159,#160));
#142=IFCPROPERTYSINGLEVALUE('Prosjekt','',IFCLABEL('e18vk'),$);
#143=IFCPROPERTYSINGLEVALUE('Entreprise','',IFCLABEL('e103'),$);
#144=IFCPROPERTYSINGLEVALUE('Byggeobjekt','',IFCLABEL('fel'),$);
#145=IFCPROPERTYSINGLEVALUE('Anleggsdel','',IFCLABEL('00'),$);
#146=IFCPROPERTYSINGLEVALUE('Fagdisiplin','',IFCLABEL('igg'),$);
#147=IFCPROPERTYSINGLEVALUE('Utarbeidet_av','',IFCLABEL('JoBern'),$);
#148=IFCPROPERTYSINGLEVALUE('Kontrollert_av','',IFCLABEL('AKVi'),$);
#149=IFCPROPERTYSINGLEVALUE('Godkjent_av','',IFCLABEL('N/A'),$);
#150=IFCPROPERTYSINGLEVALUE('MMI','',IFCLABEL('200'),$);
#151=IFCPROPERTYSINGLEVALUE('Revisjon','',IFCLABEL('A03'),$);
#152=IFCPROPERTYSINGLEVALUE('Revisjonsdato','',IFCLABEL('2023-09-04'),$);
#153=IFCPROPERTYSINGLEVALUE('Revisjon_gjelder','',IFCLABEL('Lagt inn estimerte bergpunkt ved VA-trase 2'),$);
#154=IFCPROPERTYSINGLEVALUE('Bruksstatus','',IFCLABEL('Innmålt'),$);
#155=IFCPROPERTYSINGLEVALUE('S_IMPORTFILE','',IFCLABEL('e18vk_e103_fel00_bim_mode_001_g-gf_bergmodell'),$);
#156=IFCPROPERTYSINGLEVALUE('S_IMPORTDATE','',IFCLABEL('2023-09-04'),$);
#157=IFCPROPERTYSINGLEVALUE('Layer','',IFCLABEL('NO_Bergmodell'),$);
#158=IFCPROPERTYSINGLEVALUE('Color','',IFCLABEL('1'),$);
#159=IFCPROPERTYSINGLEVALUE('Linetype','',IFCLABEL('ByLayer'),$);
#160=IFCPROPERTYSINGLEVALUE('BlockRef','',IFCLABEL('NO_Bergmodell'),$);
").unwrap();

    let client = Client::new();
    let start = std::time::Instant::now();
    // retrieve extra data around the bounding box to avoid edge artifacts
    let geotiff_data = wcs_api_call(&bbox, resolution, &client, coord_sys).await;
    println!(
        "Downloaded GeoTIFF data size: {} bytes ({:.2}s)",
        geotiff_data.len(),
        start.elapsed().as_secs_f64()
    );

    let start = std::time::Instant::now();

    // Parse GeoTIFF and extract points
    let vertices = extract_points_from_geotiff(&bbox, &geotiff_data, resolution);
    println!(
        "Extracted {} points from GeoTIFF ({:.2}s)",
        vertices.len(),
        start.elapsed().as_secs_f64()
    );

    let start = std::time::Instant::now();
    // Generate  triangulation
    let faces = triangulate_grid(bbox.num_pixels_x(resolution), bbox.num_pixels_y(resolution));
    println!(
        "Generated {} faces from triangulation ({:.2}s)",
        faces.len(),
        start.elapsed().as_secs_f64()
    );

    let start = std::time::Instant::now();

    let indices: Vec<u32> = faces.iter().flat_map(|f| vec![f[0], f[1], f[2]]).collect();

    let mesh = Mesh::new(indices, vertices);
    let mesh_simplified = mesh.simplify(0.2); // Reduce to 20% of original faces
    let mesh_compact = mesh_simplified.compact(); // Reduce to 20% of original faces
    println!(
        "Simplified mesh from {} to {} faces, and  from {} to {} vertices ({:.2}s)",
        mesh.indices.len() / 3,
        mesh_simplified.indices.len() / 3,
        mesh.vertices.len(),
        mesh_compact.vertices.len(),
        start.elapsed().as_secs_f64()
    );

    // Write points and faces to IFC
    let point_list_str = mesh_compact.write_index_list();
    let vertex_list_str = mesh_compact.write_vertex_list();

    write!(file, "{}", point_list_str).unwrap();
    write!(file, "{}", vertex_list_str).unwrap();
    write!(file, "\nENDSEC;\nEND-ISO-10303-21;").unwrap();

    Ok(file)
}

// Other helper functions

fn extract_points_from_geotiff(bbox: &BBox, data: &[u8], resolution: f32) -> Vec<Vertex> {
    use geo_types::Coord;
    use geotiff::GeoTiff;
    use std::io::Cursor;

    let mut vertices = Vec::new();

    let cursor = Cursor::new(data);
    if let Ok(reader) = GeoTiff::read(cursor) {
        for y in 0..bbox.num_pixels_y(resolution) {
            for x in 0..bbox.num_pixels_x(resolution) {
                let coord_x =
                    bbox.x1 + bbox.width() * x as f32 / bbox.num_pixels_x(resolution) as f32;
                let coord_y =
                    bbox.y1 + bbox.height() * y as f32 / bbox.num_pixels_y(resolution) as f32;

                let coord = Coord {
                    x: coord_x as f64,
                    y: coord_y as f64,
                };
                vertices.push(Vertex::new(
                    coord_x,
                    coord_y,
                    reader.get_value_at::<f32>(&coord, 0).unwrap(),
                ));
            }
        }
    }

    vertices
}

async fn wcs_api_call(bbox: &BBox, resolution: f32, client: &Client, coord_sys: usize) -> Vec<u8> {
    let padding: f32 = resolution;
    let num_pixels_x = bbox.num_pixels_x(resolution);
    let num_pixels_y = bbox.num_pixels_y(resolution);
    let url = format!(
        "http://wcs.geonorge.no/skwms1/wcs.hoyde-dtm_somlos?SERVICE=WCS&VERSION=1.0.0&REQUEST=GetCoverage&COVERAGE=las_dtm&CRS=EPSG:{}&BBOX={},{},{},{}&WIDTH={}&HEIGHT={}&FORMAT=GeoTIFF",
        coord_sys,bbox.x1-padding, bbox.y1-padding, bbox.x2+padding, bbox.y2+padding,  num_pixels_x, num_pixels_y
    );
    dbg!(&url);
    match client.get(&url).send().await {
        Ok(response) => response.bytes().await.unwrap_or_default().to_vec(),
        Err(_) => vec![],
    }
}
