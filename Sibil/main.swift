//
//  main.swift
//  Sibil
//
//  Created by Eryn Wells on 12/16/16.
//  Copyright © 2016 Eryn Wells. All rights reserved.
//

import Foundation

let l = Lexer(input: "(())")
for t in l {
    print(t)
}

