import React from "react";
import {
  Box,
  Button,
  Typography,
  Avatar,
  Step,
  Stepper,
  StepLabel,
  Fab,
} from "@mui/material";
import {
  Email,
  Phone,
  Description,
  Event,
  LocationCity,
  Create,
  Delete,
} from "@mui/icons-material";

const IndividualDocument = () => {
  const stepsData = [
    {
      label: "Document Received",
      completed: true,
      date: "Sept 1, 2023",
    },
    {
      label: "Automated Checks",
      completed: true,
      date: "Sept 2, 2023",
    },
    {
      label: "Manual Review",
      completed: false,
      date: "Expected Sept 5, 2023",
    },
    {
      label: "Final Approval",
      completed: false,
      date: "Expected Sept 10, 2023",
    },
  ];

  const activeStep = stepsData.findIndex((step) => !step.completed);

  return (
    <Box sx={{ width: "100%", p: 4 }}>
      <Typography variant="h4" gutterBottom>
        Book or Publication Title
      </Typography>

      <Stepper
        activeStep={activeStep}
        alternativeLabel
        sx={{ bgcolor: "transparent", my: 4 }}
      >
        {stepsData.map((step, index) => (
          <Step key={step.label}>
            <StepLabel
              StepIconProps={{
                completed: step.completed,
                active: index === activeStep,
              }}
            >
              {step.label}
              <Typography variant="caption" display="block" sx={{ mt: 1 }}>
                {step.date}
              </Typography>
            </StepLabel>
          </Step>
        ))}
      </Stepper>

      <Box sx={{ display: "flex", justifyContent: "space-between" }}>
        <Box>
          <Typography variant="h6" gutterBottom>
            Your Reviewer
          </Typography>
          <Box sx={{ display: "flex", alignItems: "center", mb: 1 }}>
            <Avatar sx={{ width: "25px", height: "25px" }}>J</Avatar>
            <Typography sx={{ ml: 1 }}>John Smith</Typography>
          </Box>
          <Box sx={{ display: "flex", alignItems: "center", mb: 1 }}>
            <Phone />
            <Typography sx={{ ml: 1 }}>123.456.7890</Typography>
          </Box>
          <Box sx={{ display: "flex", alignItems: "center", mb: 1 }}>
            <Email />
            <Typography sx={{ ml: 1 }}>john.smith@usccb.org</Typography>
          </Box>
          <Button
            variant="contained"
            color="primary"
            sx={{
              borderRadius: 28,
            }}
          >
            Contact
          </Button>
        </Box>
        <Box>
          <Typography variant="h6" gutterBottom>
            Document Info
          </Typography>
          <Box sx={{ display: "flex", alignItems: "center", mb: 1 }}>
            <Description />
            <Typography sx={{ ml: 1 }}>PDF</Typography>
          </Box>
          <Box sx={{ display: "flex", alignItems: "center", mb: 1 }}>
            <Event />
            <Typography sx={{ ml: 1 }}>Submitted Sept 1, 2023</Typography>
          </Box>
          <Box sx={{ display: "flex", alignItems: "center", mb: 1 }}>
            <Event />
            <Typography sx={{ ml: 1 }}>Last Modified Oct 1, 2023</Typography>
          </Box>
          <Box sx={{ display: "flex", alignItems: "center" }}>
            <LocationCity />
            <Typography sx={{ ml: 1 }}>johnsmith@ascensionpress.com</Typography>
          </Box>
        </Box>
      </Box>

      {/* Fab Buttons */}
      <Box
        sx={{
          position: "fixed",
          bottom: 32,
          right: 32,
          display: "flex",
          flexDirection: "column",
          alignItems: "flex-end",
        }}
      >
        <Fab color="primary" aria-label="edit" sx={{ m: 1 }}>
          <Create />
        </Fab>
        <Fab
          sx={{
            color: "#FFFFFF",
            backgroundColor: "#FFFFFF",
            "&:hover": {
              backgroundColor: "#FF4C4C",
            },
            m: 1,
          }}
          aria-label="delete"
        >
          <Delete
            sx={{
              color: "#e51b1b",
              "&:hover": {
                color: "#FFFFFF",
              },
            }}
          />
        </Fab>
      </Box>
    </Box>
  );
};

export default IndividualDocument;
