import 'dart:developer';

import 'package:flutter/material.dart';
import 'package:notebook/models/user.dart';
import 'package:notebook/providers/client.dart';

class CurrentUser extends ChangeNotifier {
  final Client _client;
  User? _user;

  CurrentUser(this._client) {
    reload();
  }

  User? get user => _user;

  set user(User? u) {
    _user = u;
    notifyListeners();
  }

  reload() {
    _client.getUser().then((value) {
      user = value;
    }).catchError((e) {
      log(e, level: 4);
      _client.session = null;
    });
  }
}
