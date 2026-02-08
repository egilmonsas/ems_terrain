use crate::{export::ProjectMetadata, prelude::Mesh};

// exporters/ifc/mod.rs
pub struct IfcWriter {
    buffer: String,
    project_metadata: Option<ProjectMetadata>,
    _current_id: usize,
}

impl IfcWriter {
    pub fn new(project_metadata: Option<ProjectMetadata>) -> Self {
        let mut s = Self {
            buffer: String::new(),
            project_metadata,
            _current_id: 200,
        };
        let binding = ProjectMetadata::default();
        let project_metadata = s.project_metadata.as_ref().unwrap_or(&binding);
        s.buffer.push_str(&Self::generate_header(project_metadata));
        s.buffer.push_str(&Self::generate_model_definition());
        s.buffer.push_str(&Self::generate_style());
        s
    }

    pub fn add_mesh(&mut self, mesh: &Mesh) {
        // Todo: Increment IDs for each vertex and face, and write them in the correct IFC format
        // Todo: Multiple meshes support
        // let mesh_id = self.next_id();
        // let vertex_id = self.next_id();

        let point_list = self.write_vertex_list(mesh);
        let face_list = self.write_index_list(mesh);

        self.buffer.push_str(&point_list);
        self.buffer.push_str(&face_list);
    }

    pub fn finish(self) -> Vec<u8> {
        format!("{}\nENDSEC;\nEND-ISO-10303-21;", self.buffer).into_bytes()
    }

    fn _next_id(&mut self) -> usize {
        todo!("Implement unique ID generation for IFC entities");
        // self.current_id += 1;
        // self.current_id
    }

    fn generate_header(project_metadata: &ProjectMetadata) -> String {
        let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();

        format!(
            r#"ISO-10303-21;
HEADER;
FILE_DESCRIPTION(('Option [Terrengmodell: On]'),'2;1');
FILE_NAME('','{now}',({author}),(''),'DDS_IFC v3.0','{app} build {version}','DDS');
FILE_SCHEMA(('IFC4'));
ENDSEC;
DATA;

/* APPLICATION */
#34=IFCPERSON($,$,'',$,$,$,$,$);
#35=IFCORGANIZATION($,'',$,$,$);
#36=IFCORGANIZATION($,'Data Design System AS',$,$,$);
#6=IFCPERSONANDORGANIZATION(#34,#35,$);
#7=IFCAPPLICATION(#36,'{version}','{app}','{app}');

/* PROJECT */
#1=IFCPROJECT('3rPWK2ZfD6bBH524n2KTpl',#2,'{project}',$,$,$,$,(#3),#4);
#33=IFCSITE('25Nbteh8zCUQrx_1EOgClS',$,'{site}',$,$,#51,$,$,.ELEMENT.,$,$,$,$,$);
#2=IFCOWNERHISTORY(#6,#7,$,.NOCHANGE.,$,$,$,1693812560);
#3=IFCGEOMETRICREPRESENTATIONCONTEXT($,'Model',3,1.E-05,#8,#9);
#5=IFCRELAGGREGATES('2kzQ1oOxb5l9$1Jt6usF8_',$,$,$,#1,(#33));
"#,
            now = now,
            author = project_metadata.author,
            app = project_metadata.application_name,
            version = project_metadata.application_version,
            project = project_metadata.project_name,
            site = project_metadata.site_name
        )
    }
    fn generate_model_definition() -> String {
        r#"/* MODEL */
/* MODEL */
#10=IFCGEOMETRICREPRESENTATIONSUBCONTEXT('Body','Model',*,*,*,*,#3,0.01,.MODEL_VIEW.,$);
#11=IFCMAPCONVERSION(#3,#39,0.,0.,0.,1.,0.,1.);
#51=IFCLOCALPLACEMENT($,#57);
#52=IFCRELCONTAINEDINSPATIALSTRUCTURE('2wR$JEyzD0exJeCIxHCp4q',$,$,$,(#76,#77,#78,#79,#80,#81,#82,#83,#84,#85,#86,#87,#88,#89,#90,#91,#92,#93),#33);
#54=IFCPRESENTATIONLAYERASSIGNMENT('NO_Bergmodell',$,(#53,#38),'BldBfc.3299');
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
"#.to_string()
    }

    fn generate_style() -> String {
        r#"

/* STYLE */
#164=IFCCOLOURRGB('R:255, G:0, B:0',1.,0.,0.);
#166=IFCCOLOURRGB($,0.6823529601097107,0.09019608050584793,0.);
#165=IFCSURFACESTYLESHADING(#166,0.);
#163=IFCSURFACESTYLE('Material ID: 4.988.903',.BOTH.,(#165));
#162=IFCCURVESTYLE('2b6cb502-45af-48fa-0000-000001000228',$,$,#164,$);
#161=IFCSTYLEDITEM(#53,(#162,#163),$);
"#
        .to_string()
    }

    fn write_index_list(&self, mesh: &Mesh) -> String {
        let formatted_numbers: String = mesh
            .indices
            .chunks(3)
            .map(|chunk| format!("({},{},{})", chunk[0] + 1, chunk[1] + 1, chunk[2] + 1))
            .collect::<Vec<String>>()
            .join(",");
        format!(
            "
#53=IFCTRIANGULATEDFACESET(#94,$,$,({formatted_numbers}),$);"
        )
    }

    fn write_vertex_list(&self, mesh: &Mesh) -> String {
        let formatted_numbers: String = mesh
            .vertices
            .iter()
            .map(|x| {
                format!(
                    "({:.2},{:.2},{:.2})",
                    x.position[0], x.position[1], x.position[2]
                )
            })
            .collect::<Vec<String>>()
            .join(",");

        format!(
            "
#94=IFCCARTESIANPOINTLIST3D(({formatted_numbers}));"
        )
    }
}
