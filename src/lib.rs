use futures::stream::{FuturesOrdered, StreamExt};
use indicatif::{ProgressBar, ProgressStyle}; // For the progress bar
use itertools::Itertools;
use reqwest::Client;
use serde::Deserialize;
use std::fs; // Reqwest client for HTTP requests
use std::io::prelude::*;
use std::sync::Arc;
use tokio::sync::Semaphore; // Semaphore for throttling

// API settings
const API_CALL_NUM_POINTS: usize = 50; // API restricion :-(
const API_MAX_CONCURRENT_REQUESTS: usize = 150; // Maximum concurrent API requests allowed

// IO
const OUTPUT_FILE_NAME: &str = "NVO_VBE022_Terrengmodell.ifc";

// Metadata
const NAME_AUTHOR: &str = "ems";
const NAME_APPLICATION: &str = "ems_terrain";
const VERSION_APPLICATION: &str = "0.1.0";
const NAME_PROJECT: &str = "JM - Granitten";
const NAME_SITE: &str = "Karihaugveien 22";

pub async fn generate(
    xc: f64,
    yc: f64,
    width: f64,
    height: f64,
    resolution: f64,
    coord_sys: usize,
) -> Result<Vec<u8>, reqwest::Error> {
    let x1: f64 = xc - width / 2.0;
    let x2: f64 = xc + width / 2.0;
    let y1: f64 = yc - height / 2.0;
    let y2: f64 = yc + height / 2.0;

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

    let grid = consrtuct_grid(x1, x2, y1, y2, resolution);
    let mut punkter: Vec<Punkt> = vec![];

    let vertex_list = write_vertex_list(&grid.generate_trangle_indices());
    let client = Client::new();
    let results = ordered_parallel_api_calls(grid.points, &client, coord_sys).await;
    results
        .iter()
        .for_each(|response| punkter.append(&mut response.punkter.clone()));

    let point_list = write_point_list(&punkter);

    write!(file, "{vertex_list}").unwrap();
    write!(file, "{point_list}").unwrap();

    write!(
        file,
        "
/* FOOTER */
ENDSEC;
END-ISO-10303-21;"
    )
    .unwrap();

    Ok(file)
}

#[derive(Deserialize, Debug)]
pub struct CoordResponse {
    #[allow(dead_code)]
    koordsys: usize,
    punkter: Vec<Punkt>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Punkt {
    #[allow(dead_code)]
    datakilde: Option<String>,
    #[allow(dead_code)]
    terreng: Option<String>,
    x: Option<f64>,
    y: Option<f64>,
    z: Option<f64>,
}

fn construct_query(koord_sys: usize, points: &[Punkt]) -> String {
    let mut query = String::new();
    query.push_str("https://ws.geonorge.no/hoydedata/v1/punkt?koordsys=");
    query.push_str(koord_sys.to_string().as_str());
    query.push_str("&punkter=%5B");

    for point in points.iter() {
        query.push_str("%5B");
        query.push_str(point.x.unwrap().to_string().as_str());
        query.push_str("%2C");
        query.push_str(point.y.unwrap().to_string().as_str());
        query.push_str("%5D");

        query.push_str("%2C");
    }
    query.pop();
    query.pop();
    query.pop();
    query.push_str("%5D&geojson=false");

    query
}
#[derive(Debug)]

struct Grid {
    rows: usize,
    columns: usize,
    points: Vec<Punkt>,
}

fn consrtuct_grid(x_0: f64, x_1: f64, y_0: f64, y_1: f64, resolution: f64) -> Grid {
    let mut points: Vec<Punkt> = vec![];

    let rows = ((x_1 - x_0) / resolution).floor() as usize + 1;
    let columns = ((y_1 - y_0) / resolution).floor() as usize + 1;

    for dx in 0..columns {
        for dy in 0..rows {
            points.push(Punkt {
                datakilde: None,
                terreng: None,
                x: Some(x_0 + dx as f64 * resolution),
                y: Some(y_1 - dy as f64 * resolution),
                z: None,
            });
        }
    }

    Grid {
        rows,
        columns,
        points,
    }
}

impl Grid {
    fn _size(&self) -> usize {
        self.rows * self.columns
    }
    fn _get_index(&self, row: usize, column: usize) -> usize {
        column * self.rows + row
    }

    fn generate_trangle_indices(&self) -> Vec<TriangleCoord> {
        let mut triangle_indices = vec![];
        // Loop through the grid, creating two triangles for each square
        for i in 0..(self.rows - 1) {
            for j in 0..(self.columns - 1) {
                //Calculate the indices for the current square
                let i0 = i * self.columns + j + 1;
                let i1 = i * self.columns + (j + 1) + 1;
                let i2 = (i + 1) * self.columns + j + 1;
                let i3 = (i + 1) * self.columns + (j + 1) + 1;

                // Triangle 1: (i0, i2, i1)
                triangle_indices.push(TriangleCoord {
                    v1: i0,
                    v2: i2,
                    v3: i1,
                });

                // Triangle 2: (i2, i3, i1)
                triangle_indices.push(TriangleCoord {
                    v1: i2,
                    v2: i3,
                    v3: i1,
                });
            }
        }
        triangle_indices
    }
}
struct TriangleCoord {
    v1: usize,
    v2: usize,
    v3: usize,
}

fn write_point_list(points: &[Punkt]) -> String {
    let formatted_numbers: String = points
        .iter()
        .map(|x| {
            format!(
                "({:.2},{:.2},{:.2})",
                x.x.unwrap_or_default(),
                x.y.unwrap_or_default(),
                x.z.unwrap_or_default()
            )
        })
        .collect::<Vec<String>>()
        .join(",");

    format!(
        "
#94=IFCCARTESIANPOINTLIST3D(({formatted_numbers}));"
    )
}
fn write_vertex_list(vertices: &[TriangleCoord]) -> String {
    let formatted_numbers: String = vertices
        .iter()
        .map(|x| format!("({},{},{})", x.v1, x.v2, x.v3))
        .collect::<Vec<String>>()
        .join(",");

    format!(
        "
#53=IFCTRIANGULATEDFACESET(#94,$,$,({formatted_numbers}),$);"
    )
}

async fn ordered_parallel_api_calls(
    grid_points: Vec<Punkt>,
    client: &Client,
    coord_sys: usize,
) -> Vec<CoordResponse> {
    // Final result collection
    let semaphore = Arc::new(Semaphore::new(API_MAX_CONCURRENT_REQUESTS));
    let mut tasks = FuturesOrdered::new();

    // Initialize the progress bar
    let total_chunks = (grid_points.len() + API_CALL_NUM_POINTS - 1) / API_CALL_NUM_POINTS;
    let pb = Arc::new(ProgressBar::new(total_chunks as u64));
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{elapsed_precise}] [{wide_bar}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("█▇▆▅▄▃▂▁  "),
    );
    pb.set_message("Processing API calls");

    // Collect asynchronous tasks in an ordered set for ordered results
    grid_points
        .into_iter()
        .chunks(API_CALL_NUM_POINTS) // Chunk the points into API_CALL_NUM_POINTS size
        .into_iter()
        .for_each(|chunk| {
            let vec: Vec<Punkt> = chunk.collect(); // Collect chunk into Vec<Punkt>
            let query = construct_query(coord_sys, &vec); // Construct the API query
            let permit = Arc::clone(&semaphore);
            let client = client.clone();
            let pb = Arc::clone(&pb);

            tasks.push_back(async move {
                let _permit = permit.acquire().await.unwrap();

                // Make the API call
                let response = client
                    .get(&query)
                    .send()
                    .await?
                    .json::<CoordResponse>()
                    .await?;

                // Update the progress bar after the task completes
                pb.inc(1);

                Ok(response) as Result<CoordResponse, reqwest::Error> // Return the result
            });
        });

    // Execute all the asynchronous requests in the order of submission and gather results
    tasks
        .map(|t| t.unwrap())
        .collect::<Vec<CoordResponse>>()
        .await
}
