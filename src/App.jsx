import React from 'react';
import Dashboard from './components/dashboard/Dashboard.jsx';

import { Tab, Tabs, TabList, TabPanel } from 'react-tabs';
import tabStyles from 'react-tabs/style/react-tabs.css';

console.log(tabStyles);

export default function App() {
    return  <Tabs
                selectedTabClassName={tabStyles.reactTabsTabSelected}
                selectedTabPanelClassName={tabStyles.reactTabsTabPanelSelected}
                disabledTabClassName={tabStyles.reactTabsTabDisabled}
                forceRenderTabPanel={true}
            >
                <TabList className={tabStyles.reactTabsTabList}>
                    <Tab className={tabStyles.reactTabsTab}>Grover's Algorithm</Tab>
                    <Tab className={tabStyles.reactTabsTab}>Custom Circuits</Tab>
                </TabList>

                <TabPanel className={tabStyles.reactTabsTabPanel}>
                    <Dashboard bits={4} />
                </TabPanel>
                <TabPanel className={tabStyles.reactTabsTabPanel}>
                    <h2>Any content 2</h2>
                </TabPanel>
            </Tabs>;
}
