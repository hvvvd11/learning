import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart';
import 'package:store_mobile_app/views/buyers/nav_screens/account_screen.dart';
import 'package:store_mobile_app/views/buyers/nav_screens/cart_screen.dart';
import 'package:store_mobile_app/views/buyers/nav_screens/category_screen.dart';
import 'package:store_mobile_app/views/buyers/nav_screens/home_screen.dart';
import 'package:store_mobile_app/views/buyers/nav_screens/search_screen.dart';
import 'package:store_mobile_app/views/buyers/nav_screens/store_screen.dart';

class MainScreen extends StatefulWidget {
  const MainScreen({super.key});

  @override
  State<MainScreen> createState() => _MainScreenState();
}

class _MainScreenState extends State<MainScreen> {
  int _pageIndex = 0;

  final List<Widget> _screens = [
    const HomeScreen(),
    const AccountScreen(),
    const CartScreen(),
    const CategoryScreen(),
    const SearchScreen(),
    const StoreScreen(),
  ];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: _screens[_pageIndex],
      bottomNavigationBar: BottomNavigationBar(
          currentIndex: _pageIndex,
          onTap: (value) {
            setState(() {
              _pageIndex = value;
            });
          },
          unselectedItemColor: Colors.black,
          selectedItemColor: Colors.yellow,
          items: [
            const BottomNavigationBarItem(icon: Icon(CupertinoIcons.home), label: "Home"),
            BottomNavigationBarItem(
                icon: SvgPicture.asset(
                  "assets/icons/explore.svg",
                  width: 20,
                ),
                label: "Explore"),
            BottomNavigationBarItem(
                icon: SvgPicture.asset(
                  "assets/icons/shop.svg",
                  width: 20,
                ),
                label: "Store"),
            BottomNavigationBarItem(
                icon: SvgPicture.asset(
                  "assets/icons/cart.svg",
                  width: 20,
                ),
                label: "Cart"),
            BottomNavigationBarItem(
                icon: SvgPicture.asset(
                  "assets/icons/search.svg",
                  width: 20,
                ),
                label: "Search"),
            BottomNavigationBarItem(
                icon: SvgPicture.asset(
                  "assets/icons/account.svg",
                  width: 20,
                ),
                label: "Account"),
          ]),
    );
  }
}
