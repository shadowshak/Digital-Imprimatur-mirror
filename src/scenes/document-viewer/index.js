import React, { useState } from "react";
import {
  Box,
  AppBar,
  Toolbar,
  IconButton,
  Typography,
  Tabs,
  Tab,
  List,
  ListItem,
  Card,
  CardContent,
  CardActions,
  CardHeader,
  Grid,
  Drawer,
  Divider
} from "@mui/material";
import "./PdfViewer.css";
import file from "./book.pdf";
import Pdf from "@mikecousins/react-pdf";

//This config needed only if pdf.worker.js is not available (/pdf.worker.js)
import { pdfjs } from "react-pdf";
pdfjs.GlobalWorkerOptions.workerSrc = `//cdnjs.cloudflare.com/ajax/libs/pdf.js/2.1.266/pdf.worker.js`;

function DocumentViewer(props) {
  const [page, setPage] = useState(1);
  const [pageNb, setPageNb] = useState(null);

  const onDocumentLoadSuccess = ({ numPages }) => {
    setPageNb(numPages);
  };

  const onPrevPage = () => {
    if (page > 1) {
      setPage(page - 1);
    }
  };

  return (
    <>
      <AppBar position="static">
        <Toolbar>
          <Typography variant="h6" component="div">
            Document Viewing
          </Typography>
        </Toolbar>
      </AppBar>
      <Toolbar />
      <Box sx={{ display: "flex", justifyContent: "center" }}>
        <div className="container">
          <div className="actions">
            <button
              disabled={page === 1}
              onClick={onPrevPage}
              sx={{ mr: "1rem" }}
            >
              Prev
            </button>
            <Typography variant="body1">
              Page {page} of {pageNb}
            </Typography>
            <button
              disabled={page === pageNb}
              onClick={() => setPage(page + 1)}
              sx={{ ml: "1rem" }}
            >
              Next
            </button>
          </div>
          <Pdf
            file={file}
            page={page}
            onDocumentLoadSuccess={onDocumentLoadSuccess}
            className="pdf-canvas"
          >
            {({ pdfDocument, pdfPage, canvas }) => (
              <>
                {!pdfDocument && <span>Loading...</span>}
                {canvas}
              </>
            )}
          </Pdf>
        </div>
      </Box>
    </>
  );
}

export default DocumentViewer;
