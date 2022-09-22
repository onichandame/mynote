import 'package:flutter/material.dart';
import 'package:notebook/components/layout.dart';
import 'package:notebook/providers/current_user.dart';
import 'package:notebook/screens/routes.dart';
import 'package:provider/provider.dart';

class DashboardLayout extends StatelessWidget {
  final String title;
  final Widget body;
  final int index;

  const DashboardLayout(
      {Key? key, required this.title, required this.body, required this.index})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Layout(
      title: title,
      body: Consumer<CurrentUser?>(
        builder: (context, value, child) =>
            value?.user == null ? const _Guest() : body,
      ),
      bottomNavigationBar: BottomNavigationBar(
        currentIndex: index,
        onTap: (value) {
          Navigator.of(context).pushNamed(dashboardMap[value].route);
        },
        items: const [
          BottomNavigationBarItem(
            icon: Icon(Icons.list_sharp),
            label: 'Todo',
          ),
          BottomNavigationBarItem(icon: Icon(Icons.note_sharp), label: 'Note'),
          BottomNavigationBarItem(
              icon: Icon(Icons.report_sharp), label: 'Report')
        ],
      ),
    );
  }
}

class _Guest extends StatelessWidget {
  const _Guest({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Column(mainAxisAlignment: MainAxisAlignment.center, children: [
        const Text('Welcome to your private notebook'),
        Wrap(
          alignment: WrapAlignment.center,
          spacing: 10,
          children: const [
            _GuestRoute(routeLogin, 'Log In'),
            _GuestRoute(routeSignup, 'Sign Up')
          ]
              .map((route) => TextButton(
                  onPressed: () {
                    Navigator.of(context).pushNamed(route.route);
                  },
                  child: Text(route.title)))
              .toList(),
        )
      ]),
    );
  }
}

class _GuestRoute {
  final String route;
  final String title;
  const _GuestRoute(this.route, this.title);
}

class _DashboardRoute {
  final String route;
  final Widget icon;
  final String title;
  const _DashboardRoute(this.route, this.icon, this.title);
}

const dashboardMap = [
  _DashboardRoute(routeDashboardTodos, Icon(Icons.list_sharp), 'Todo'),
  _DashboardRoute(routeDashboardNotes, Icon(Icons.note_sharp), 'Note'),
  _DashboardRoute(routeDashboardReports, Icon(Icons.report_sharp), 'Report')
];
