// drawing - A module for creating the Excel Drawing.xml file.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

mod tests;

use crate::{xmlwriter::XMLWriter, ObjectMovement, Url};

pub struct Drawing {
    pub(crate) writer: XMLWriter,
    pub(crate) drawings: Vec<DrawingInfo>,
}

impl Drawing {
    // -----------------------------------------------------------------------
    // Public (and crate public) methods.
    // -----------------------------------------------------------------------

    // Create a new Drawing struct.
    pub fn new() -> Drawing {
        let writer = XMLWriter::new();

        Drawing {
            writer,
            drawings: vec![],
        }
    }

    // -----------------------------------------------------------------------
    // XML assembly methods.
    // -----------------------------------------------------------------------

    // Assemble and write the XML file.
    pub fn assemble_xml_file(&mut self) {
        self.writer.xml_declaration();

        // Write the xdr:wsDr element.
        self.write_ws_dr();

        for (index, drawing) in self.drawings.clone().iter().enumerate() {
            // Write the xdr:twoCellAnchor element.
            self.write_two_cell_anchor((index + 1) as u32, drawing);
        }

        // Close the end tag.
        self.writer.xml_end_tag("xdr:wsDr");
    }

    // Write the <xdr:wsDr> element.
    fn write_ws_dr(&mut self) {
        let attributes = [
            (
                "xmlns:xdr",
                "http://schemas.openxmlformats.org/drawingml/2006/spreadsheetDrawing",
            ),
            (
                "xmlns:a",
                "http://schemas.openxmlformats.org/drawingml/2006/main",
            ),
        ];

        self.writer.xml_start_tag("xdr:wsDr", &attributes);
    }

    // Write the <xdr:twoCellAnchor> element.
    fn write_two_cell_anchor(&mut self, index: u32, drawing_info: &DrawingInfo) {
        let mut attributes = vec![];

        match drawing_info.object_movement {
            ObjectMovement::MoveButDontSizeWithCells => {
                attributes.push(("editAs", "oneCell".to_string()));
            }
            ObjectMovement::DontMoveOrSizeWithCells => {
                attributes.push(("editAs", "absolute".to_string()));
            }
            ObjectMovement::MoveAndSizeWithCells | ObjectMovement::MoveAndSizeWithCellsAfter => (),
        }

        self.writer.xml_start_tag("xdr:twoCellAnchor", &attributes);

        // Write the xdr:from and xdr:to elements
        self.write_from(&drawing_info.from);
        self.write_to(&drawing_info.to);

        match drawing_info.drawing_type {
            DrawingType::Image => self.write_pic(index, drawing_info),
            DrawingType::Chart => self.write_graphic_frame(index, drawing_info),
        }

        self.writer.xml_empty_tag_only("xdr:clientData");
        self.writer.xml_end_tag("xdr:twoCellAnchor");
    }

    // Write the <xdr:from> element.
    fn write_from(&mut self, coords: &DrawingCoordinates) {
        self.writer.xml_start_tag_only("xdr:from");

        self.writer
            .xml_data_element_only("xdr:col", &coords.col.to_string());
        self.writer
            .xml_data_element_only("xdr:colOff", &coords.col_offset.to_string());
        self.writer
            .xml_data_element_only("xdr:row", &coords.row.to_string());
        self.writer
            .xml_data_element_only("xdr:rowOff", &coords.row_offset.to_string());

        self.writer.xml_end_tag("xdr:from");
    }

    // Write the <xdr:to> element.
    fn write_to(&mut self, coords: &DrawingCoordinates) {
        self.writer.xml_start_tag_only("xdr:to");

        self.writer
            .xml_data_element_only("xdr:col", &coords.col.to_string());
        self.writer
            .xml_data_element_only("xdr:colOff", &coords.col_offset.to_string());
        self.writer
            .xml_data_element_only("xdr:row", &coords.row.to_string());
        self.writer
            .xml_data_element_only("xdr:rowOff", &coords.row_offset.to_string());

        self.writer.xml_end_tag("xdr:to");
    }

    // Write the <xdr:pic> element.
    fn write_pic(&mut self, index: u32, drawing_info: &DrawingInfo) {
        self.writer.xml_start_tag_only("xdr:pic");

        // Write the xdr:nvPicPr element.
        self.write_nv_pic_pr(index, drawing_info);

        // Write the xdr:blipFill element.
        self.write_blip_fill(drawing_info.rel_id);

        // Write the xdr:spPr element.
        self.write_sp_pr(drawing_info);

        self.writer.xml_end_tag("xdr:pic");
    }

    // Write the <xdr:nvPicPr> element.
    fn write_nv_pic_pr(&mut self, index: u32, drawing_info: &DrawingInfo) {
        self.writer.xml_start_tag_only("xdr:nvPicPr");

        // Write the xdr:cNvPr element.
        self.write_c_nv_pr(index, drawing_info, "Picture");

        // Write the xdr:cNvPicPr element.
        self.writer.xml_start_tag_only("xdr:cNvPicPr");
        self.write_a_pic_locks();
        self.writer.xml_end_tag("xdr:cNvPicPr");

        self.writer.xml_end_tag("xdr:nvPicPr");
    }

    // Write the <xdr:cNvPr> element.
    fn write_c_nv_pr(&mut self, index: u32, drawing_info: &DrawingInfo, name: &str) {
        let id = index + 1;
        let mut name = format!("{name} {index}");

        if !drawing_info.name.is_empty() {
            name.clone_from(&drawing_info.name);
        }

        let mut attributes = vec![("id", id.to_string()), ("name", name)];

        if !drawing_info.description.is_empty() {
            attributes.push(("descr", drawing_info.description.clone()));
        }

        if drawing_info.decorative || drawing_info.url.is_some() {
            self.writer.xml_start_tag("xdr:cNvPr", &attributes);

            if let Some(hyperlink) = &drawing_info.url {
                // Write the a:hlinkClick element.
                self.write_hyperlink(hyperlink);
            }

            if drawing_info.decorative {
                self.write_decorative();
            }

            self.writer.xml_end_tag("xdr:cNvPr");
        } else {
            self.writer.xml_empty_tag("xdr:cNvPr", &attributes);
        }
    }

    // Write the decorative sub elements.
    fn write_decorative(&mut self) {
        self.writer.xml_start_tag_only("a:extLst");

        let attributes = [("uri", "{FF2B5EF4-FFF2-40B4-BE49-F238E27FC236}")];
        self.writer.xml_start_tag("a:ext", &attributes);

        let attributes = [
            (
                "xmlns:a16",
                "http://schemas.microsoft.com/office/drawing/2014/main",
            ),
            ("id", "{00000000-0008-0000-0000-000002000000}"),
        ];
        self.writer.xml_empty_tag("a16:creationId", &attributes);

        self.writer.xml_end_tag("a:ext");

        let attributes = [("uri", "{C183D7F6-B498-43B3-948B-1728B52AA6E4}")];
        self.writer.xml_start_tag("a:ext", &attributes);

        let attributes = [
            (
                "xmlns:adec",
                "http://schemas.microsoft.com/office/drawing/2017/decorative",
            ),
            ("val", "1"),
        ];
        self.writer.xml_empty_tag("adec:decorative", &attributes);

        self.writer.xml_end_tag("a:ext");
        self.writer.xml_end_tag("a:extLst");
    }

    // Write the <a:hlinkClick> element.
    fn write_hyperlink(&mut self, hyperlink: &Url) {
        let mut attributes = vec![
            (
                "xmlns:r",
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships".to_string(),
            ),
            ("r:id", format!("rId{}", hyperlink.rel_id)),
        ];

        if !hyperlink.tool_tip.is_empty() {
            attributes.push(("tooltip", hyperlink.tool_tip.clone()));
        }

        self.writer.xml_empty_tag("a:hlinkClick", &attributes);
    }

    // Write the <a:picLocks> element.
    fn write_a_pic_locks(&mut self) {
        let attributes = [("noChangeAspect", "1")];

        self.writer.xml_empty_tag("a:picLocks", &attributes);
    }

    // Write the <xdr:blipFill> element.
    fn write_blip_fill(&mut self, index: u32) {
        self.writer.xml_start_tag_only("xdr:blipFill");

        // Write the a:blip element.
        self.write_a_blip(index);

        self.writer.xml_start_tag_only("a:stretch");
        self.writer.xml_empty_tag_only("a:fillRect");
        self.writer.xml_end_tag("a:stretch");

        self.writer.xml_end_tag("xdr:blipFill");
    }

    // Write the <a:blip> element.
    fn write_a_blip(&mut self, index: u32) {
        let attributes = [
            (
                "xmlns:r",
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships".to_string(),
            ),
            ("r:embed", format!("rId{index}")),
        ];

        self.writer.xml_empty_tag("a:blip", &attributes);
    }

    // Write the <xdr:spPr> element.
    fn write_sp_pr(&mut self, drawing_info: &DrawingInfo) {
        self.writer.xml_start_tag_only("xdr:spPr");
        self.writer.xml_start_tag_only("a:xfrm");

        // Write the a:off element.
        self.write_a_off(drawing_info);

        // Write the a:ext element.
        self.write_a_ext(drawing_info);

        self.writer.xml_end_tag("a:xfrm");

        // Write the a:prstGeom element.
        self.write_a_prst_geom();

        self.writer.xml_end_tag("xdr:spPr");
    }

    // Write the <a:off> element.
    fn write_a_off(&mut self, drawing_info: &DrawingInfo) {
        let attributes = [
            ("x", drawing_info.col_absolute.to_string()),
            ("y", drawing_info.row_absolute.to_string()),
        ];

        self.writer.xml_empty_tag("a:off", &attributes);
    }

    // Write the <a:ext> element.
    fn write_a_ext(&mut self, drawing_info: &DrawingInfo) {
        let attributes = [
            ("cx", drawing_info.width.to_string()),
            ("cy", drawing_info.height.to_string()),
        ];

        self.writer.xml_empty_tag("a:ext", &attributes);
    }

    // Write the <a:prstGeom> element.
    fn write_a_prst_geom(&mut self) {
        let attributes = [("prst", "rect")];

        self.writer.xml_start_tag("a:prstGeom", &attributes);
        self.writer.xml_empty_tag_only("a:avLst");
        self.writer.xml_end_tag("a:prstGeom");
    }

    // Write the <xdr:graphicFrame> element.
    fn write_graphic_frame(&mut self, index: u32, drawing_info: &DrawingInfo) {
        let attributes = [("macro", "")];

        self.writer.xml_start_tag("xdr:graphicFrame", &attributes);

        // Write the xdr:nvGraphicFramePr element.
        self.write_nv_graphic_frame_pr(index, drawing_info);

        // Write the xdr:xfrm element.
        self.write_xfrm();

        // Write the a:graphic element.
        self.write_a_graphic(drawing_info.rel_id);

        self.writer.xml_end_tag("xdr:graphicFrame");
    }

    // Write the <xdr:nvGraphicFramePr> element.
    fn write_nv_graphic_frame_pr(&mut self, index: u32, drawing_info: &DrawingInfo) {
        self.writer.xml_start_tag_only("xdr:nvGraphicFramePr");

        // Write the xdr:cNvPr element.
        self.write_c_nv_pr(index, drawing_info, "Chart");

        // Write the xdr:cNvGraphicFramePr element.
        self.write_c_nv_graphic_frame_pr();

        self.writer.xml_end_tag("xdr:nvGraphicFramePr");
    }

    // Write the <xdr:cNvGraphicFramePr> element.
    fn write_c_nv_graphic_frame_pr(&mut self) {
        self.writer.xml_empty_tag_only("xdr:cNvGraphicFramePr");
    }

    // Write the <xdr:xfrm> element.
    fn write_xfrm(&mut self) {
        self.writer.xml_start_tag_only("xdr:xfrm");

        // Write the a:off element.
        self.write_chart_a_off();

        // Write the a:ext element.
        self.write_chart_a_ext();

        self.writer.xml_end_tag("xdr:xfrm");
    }

    // Write the <a:off> element.
    fn write_chart_a_off(&mut self) {
        let attributes = [("x", "0"), ("y", "0")];

        self.writer.xml_empty_tag("a:off", &attributes);
    }

    // Write the <a:ext> element.
    fn write_chart_a_ext(&mut self) {
        let attributes = [("cx", "0"), ("cy", "0")];

        self.writer.xml_empty_tag("a:ext", &attributes);
    }

    // Write the <a:graphic> element.
    fn write_a_graphic(&mut self, index: u32) {
        self.writer.xml_start_tag_only("a:graphic");

        // Write the a:graphicData element.
        self.write_a_graphic_data(index);

        self.writer.xml_end_tag("a:graphic");
    }

    // Write the <a:graphicData> element.
    fn write_a_graphic_data(&mut self, index: u32) {
        let attributes = [(
            "uri",
            "http://schemas.openxmlformats.org/drawingml/2006/chart",
        )];

        self.writer.xml_start_tag("a:graphicData", &attributes);

        // Write the c:chart element.
        self.write_chart(index);

        self.writer.xml_end_tag("a:graphicData");
    }

    // Write the <c:chart> element.
    fn write_chart(&mut self, index: u32) {
        let attributes = [
            (
                "xmlns:c",
                "http://schemas.openxmlformats.org/drawingml/2006/chart".to_string(),
            ),
            (
                "xmlns:r",
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships".to_string(),
            ),
            ("r:id", format!("rId{index}")),
        ];

        self.writer.xml_empty_tag("c:chart", &attributes);
    }
}

// -----------------------------------------------------------------------
// Helper enums/structs/functions.
// -----------------------------------------------------------------------
#[derive(Clone)]
pub(crate) struct DrawingCoordinates {
    pub(crate) col: u32,
    pub(crate) row: u32,
    pub(crate) col_offset: f64,
    pub(crate) row_offset: f64,
}

#[derive(Clone)]
pub(crate) struct DrawingInfo {
    pub(crate) from: DrawingCoordinates,
    pub(crate) to: DrawingCoordinates,
    pub(crate) col_absolute: u64,
    pub(crate) row_absolute: u64,
    pub(crate) width: f64,
    pub(crate) height: f64,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) decorative: bool,
    pub(crate) object_movement: ObjectMovement,
    pub(crate) rel_id: u32,
    pub(crate) drawing_type: DrawingType,
    pub(crate) url: Option<Url>,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum DrawingType {
    Image,
    Chart,
}

// Trait for object such as Images and Charts that translate to a Drawing object.
pub(crate) trait DrawingObject {
    fn x_offset(&self) -> u32;
    fn y_offset(&self) -> u32;
    fn width_scaled(&self) -> f64;
    fn height_scaled(&self) -> f64;
    fn object_movement(&self) -> ObjectMovement;
    fn name(&self) -> String;
    fn alt_text(&self) -> String;
    fn decorative(&self) -> bool;
    fn drawing_type(&self) -> DrawingType;
}
