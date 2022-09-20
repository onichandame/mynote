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

  reload() async {
    try {
      user = await _client.getUser();
    } catch (e) {
      log(e.toString(), level: 4);
      _client.session = null;
    }
  }

  Future<void> updateName(String name) async {
    await _client.updateProfileName(name);
    await reload();
  }

  Future<void> updateAvatar(String url) async {
    await _client.updateProfileAvatar(url);
    await reload();
  }

  Future<void> updateEmail(String email) async {
    await _client.updateProfileEmail(email);
    await reload();
  }
}
