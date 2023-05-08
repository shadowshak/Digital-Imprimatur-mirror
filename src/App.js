import React from "react";
import { BrowserRouter as Router, Route, Routes } from "react-router-dom";
import CssBaseline from "@mui/material/CssBaseline";
import Document from "./scenes/document/Index";
import DocumentViewer from "./scenes/document-viewer/index";
import Home from "./scenes/home/Index";
import Login from "./scenes/login/Index";

function App() {
  return (
    <Router>
      <CssBaseline />
      <Routes>
        <Route path="/reviewer/home" element={<Home role="reviewer" />} />
        <Route path="/publisher/home" element={<Home role="publisher" />} />
        <Route path="/login" element={<Login />} />
        <Route
          path="reviewer/document"
          element={<Document role="reviewer" />}
        />
        <Route
          path="publisher/document"
          element={<Document role="publisher" />}
        />
        <Route
          path="publisher/document-viewer"
          element={<DocumentViewer role={"publisher"} />}
        />
      </Routes>
    </Router>
  );
}

export default App;
