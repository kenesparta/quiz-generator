import React from 'react';
import { Grid, Paper, Typography, Box } from '@mui/material';
import TrendingUpIcon from '@mui/icons-material/TrendingUp';
import PeopleIcon from '@mui/icons-material/People';

export const DashboardCards = () => {
  return (
    <Grid container spacing={3}>
      <Grid>
        <Paper
          sx={{
            p: 2,
            display: 'flex',
            flexDirection: 'column',
            height: 140,
          }}
        >
          <Box sx={{ display: 'flex', justifyContent: 'space-between' }}>
            <Box>
              <Typography component="h2" variant="h6" color="primary" gutterBottom>
                Sales
              </Typography>
              <Typography component="p" variant="h4">
                $3,024
              </Typography>
            </Box>
            <TrendingUpIcon color="primary" sx={{ fontSize: 40 }} />
          </Box>
          <Typography color="text.secondary" sx={{ flex: 1 }}>
            15% increase from last month
          </Typography>
        </Paper>
      </Grid>

      <Grid>
        <Paper
          sx={{
            p: 2,
            display: 'flex',
            flexDirection: 'column',
            height: 140,
          }}
        >
          <Box sx={{ display: 'flex', justifyContent: 'space-between' }}>
            <Box>
              <Typography component="h2" variant="h6" color="primary" gutterBottom>
                Users
              </Typography>
              <Typography component="p" variant="h4">
                4,209
              </Typography>
            </Box>
            <PeopleIcon color="primary" sx={{ fontSize: 40 }} />
          </Box>
          <Typography color="text.secondary" sx={{ flex: 1 }}>
            12% increase from last week
          </Typography>
        </Paper>
      </Grid>
    </Grid>
  );
}