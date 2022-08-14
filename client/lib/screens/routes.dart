import 'package:flutter/material.dart';
import 'package:notebook/screens/signup.dart';

import 'home.dart';

Map<String, WidgetBuilder> getRoutes() {
  return {
    '/home': (_) => const HomeScreen(),
    '/signup': (_) => const SignupScreen(),
  };
}
