import { Box, Tab, Tabs } from "@mui/material";
import { FC, useState } from "react";

const TabPanel: FC<{
  index: number | string;
  currentIndex: number | string;
}> = ({ index, children, currentIndex }) => {
  return (
    <div
      role="tabpanel"
      hidden={currentIndex !== index}
      id={`settings-panel-${index}`}
      aria-labelledby={`settings-tab-${index}`}
    >
      {index === currentIndex && <Box sx={{ p: 3 }}>{children}</Box>}
    </div>
  );
};

const IndexedTab: FC<{ index: number | string }> = ({ index }) => {
  return (
    <Tab
      label="Profile"
      id={`settings-tab-${index}`}
      aria-controls={`settings-panel-${index}`}
    />
  );
};

export const Settings: FC = () => {
  const [currentTab, setCurrentTab] = useState(0);
  return (
    <Box sx={{ width: `auto` }}>
      <Box sx={{ borderBottom: 1, borderColor: `divider` }}>
        <Tabs
          onChange={(_, newIndex) => {
            setCurrentTab(newIndex);
          }}
        >
          <IndexedTab index={0} />
        </Tabs>
      </Box>
      <TabPanel index={0} currentIndex={currentTab}>
        profile
      </TabPanel>
    </Box>
  );
};
