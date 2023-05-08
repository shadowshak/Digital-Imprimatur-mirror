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
  Divider,
} from "@mui/material";
import { GrammarlyEditorPlugin } from "@grammarly/editor-sdk-react";
import CheckCircleOutlineIcon from "@mui/icons-material/CheckCircleOutline";
import { saveAs } from "file-saver";
import { demoText } from "./demo";
import "./document.css";

function Document({ role }) {
  const [text, setText] = useState("");
  const [showSuggestions, setShowSuggestions] = useState(false);

  const handleFileUpload = (event) => {
    const file = event.target.files[0];
    const reader = new FileReader();
    reader.onload = (e) => {
      setText(e.target.result);
      document.querySelector("#text").value = e.target.result; // Update the textarea value
    };
    reader.readAsText(file);
  };

  const handleSave = async () => {
    const textToSave = document.querySelector("#text").value;
    const blob = new Blob([textToSave], { type: "text/plain;charset=utf-8" });
    saveAs(blob, "edited_document.txt");
  };

  const handleToggleSuggestions = () => {
    setShowSuggestions(!showSuggestions);
  };

  return (
    <>
      <AppBar>
        <Toolbar>
          <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
            Edit Document:Fast and Precise Type Checking for JavaScript
          </Typography>
        </Toolbar>
      </AppBar>
      <Toolbar />
      <Box
        sx={{
          backgroundColor: "#dee6ed",
          height: "calc(100vh - 64px)", // Subtract the height of the AppBar
          overflow: "hidden", // Hide any overflow content
        }}
      >
        <AppBar>{/* App bar content */}</AppBar>
        {/* Other toolbar */}
        <Box height="100%">
          {/* Add a wrapper div to ensure the iframe takes up the full height */}
          <iframe
            title="Fast and Precise Type Checking for JavaScript"
            src="https://agentcooper.github.io/react-pdf-highlighter/"
            width="100%"
            height="100%"
            style={{ border: "none" }} // Remove iframe border
          ></iframe>
        </Box>
      </Box>
    </>
  );
}

export default Document;
