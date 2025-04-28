import React from 'react';
import { List, ListItem, ListItemIcon, ListItemText, Divider } from '@mui/material';
import DashboardIcon from '@mui/icons-material/Dashboard';
import PeopleAltRoundedIcon from '@mui/icons-material/PeopleAltRounded';
import BarChartIcon from '@mui/icons-material/BarChart';
import SettingsIcon from '@mui/icons-material/Settings';
import Link from 'next/link';

export const SidebarMenu = () => {
  return (
    <>
      <List style={{backgroundColor: ''}}>
        <Link href="/dashboard" passHref style={{ textDecoration: 'none', color: 'inherit' }}>
          <ListItem>
            <ListItemIcon>
              <DashboardIcon />
            </ListItemIcon>
            <ListItemText primary="Dashboard" />
          </ListItem>
        </Link>
        <Link href="/dashboard/postulante" passHref style={{ textDecoration: 'none', color: 'inherit' }}>
          <ListItem >
            <ListItemIcon>
              <PeopleAltRoundedIcon />
            </ListItemIcon>
            <ListItemText primary="Users" />
          </ListItem>
        </Link>
        <Link href="/dashboard/evaluacion" passHref style={{ textDecoration: 'none', color: 'inherit' }}>
          <ListItem>
            <ListItemIcon>
              <BarChartIcon />
            </ListItemIcon>
            <ListItemText primary="Reports" />
          </ListItem>
        </Link>
      </List>
      <Divider />
      <List>
        <Link href="/dashboard/respuestas" passHref style={{ textDecoration: 'none', color: 'inherit' }}>
          <ListItem>
            <ListItemIcon>
              <SettingsIcon />
            </ListItemIcon>
            <ListItemText primary="Settings" />
          </ListItem>
        </Link>
      </List>
    </>
  );
}