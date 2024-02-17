import 'package:flutter/material.dart';
import 'package:store_mobile_app/views/buyers/nav_screens/widgets/banner_widget.dart';
import 'package:store_mobile_app/views/buyers/nav_screens/widgets/search_input.dart';
import 'package:store_mobile_app/views/buyers/nav_screens/widgets/welcome_home_widget.dart';

class HomeScreen extends StatelessWidget {
  const HomeScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return const Column(
      children: [
        WelcomeTextWidget(),
        SizedBox(height: 5),
        SearchInputWidget(),
        BannerWidget(),
      ],
    );
  }
}
